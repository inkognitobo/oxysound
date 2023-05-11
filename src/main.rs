mod args;
mod error;
mod playlist;
mod prelude;
mod youtube_api;

use crate::args::{Arguments, Operation};
use crate::playlist::{load_playlist, Playlist, Video};
use crate::prelude::*;
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    const FILE_PATH: &str = "./";

    let arguments = Arguments::parse();

    let playlist = match &arguments.operation {
        Operation::Create(create_args) => {
            let mut playlist = match &create_args.ids {
                Some(ids) => {
                    let videos = ids
                        .iter()
                        .map(|id| id.to_string())
                        .map(Video::from)
                        .collect();
                    Playlist::new_with_videos(&create_args.playlist_title, videos)
                }
                None => Playlist::new(&create_args.playlist_title),
            };
            playlist.fetch_metadata().await?;
            playlist
        }
        Operation::Add(args) => {
            let mut playlist = load_playlist(&args.playlist_title, FILE_PATH)?;
            playlist.add_videos(&args.ids);
            playlist.fetch_metadata().await?;
            playlist
        }
        Operation::Remove(args) => {
            let mut playlist = load_playlist(&args.playlist_title, FILE_PATH)?;
            playlist.remove_videos(&args.ids);
            playlist
        }
        Operation::Print(args) => match (&args.playlist_title, &args.ids) {
            (Some(playlist_title), None) => load_playlist(playlist_title, FILE_PATH)?,
            (None, Some(ids)) => {
                let mut playlist = Playlist::default();
                playlist.add_videos(ids);
                playlist
            }
            _ => Playlist::default(),
            // Not reachable, because `PrintArgs.playlist_title` and `PrintArgs.ids` are mutually exclusive
        },
    };

    println!("Playlist URL:\n{:?}", playlist.url());

    // Save the playlist depending on the selected operation
    match arguments.operation {
        Operation::Print(_) => (),
        _ => playlist.save_playlist(FILE_PATH)?,
    }

    Ok(())
}
