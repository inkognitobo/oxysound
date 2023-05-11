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

    let playlist = match arguments.operation {
        Operation::Create(create_args) => {
            let mut playlist = match create_args.ids {
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
        Operation::Add(add_args) => {
            let mut playlist = load_playlist(&add_args.playlist_title, FILE_PATH)?;
            playlist.add_videos(&add_args.ids);
            playlist.fetch_metadata().await?;
            playlist
        }
        Operation::Remove(remove_args) => {
            let mut playlist = load_playlist(&remove_args.playlist_title, FILE_PATH)?;
            playlist.remove_videos(&remove_args.ids);
            playlist
        }
    };

    playlist.save_playlist(FILE_PATH)?;

    println!("Playlist URL:\n{:?}", playlist.url());

    Ok(())
}
