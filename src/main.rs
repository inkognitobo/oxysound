mod args;
mod config;
mod error;
mod playlist;
mod prelude;
mod utils;
mod youtube_api;

use crate::args::{Arguments, Operation};
use crate::config::Config;
use crate::playlist::{load_playlist, Playlist};
use clap::Parser;

#[tokio::main]
async fn main() -> std::result::Result<(), anyhow::Error> {
    let config: Config = confy::load::<Config>("oxysound", "config")?.assert_values()?;

    let arguments = Arguments::parse();

    let playlist = match &arguments.operation {
        Operation::Add(args) => {
            let mut playlist = match load_playlist(&args.playlist_title, &config.save_directory)? {
                Some(playlist) => playlist,
                None => Playlist::new(&args.playlist_title),
            };
            playlist.add_videos(&args.ids);
            playlist.fetch_metadata().await?;
            playlist
        }
        Operation::Remove(args) => {
            let mut playlist = match load_playlist(&args.playlist_title, &config.save_directory)? {
                Some(playlist) => playlist,
                None => Playlist::new(&args.playlist_title),
            };
            playlist.remove_videos(&args.ids);
            playlist
        }
        Operation::Print(args) => match (&args.playlist_title, &args.ids) {
            (Some(playlist_title), None) => {
                match load_playlist(playlist_title, &config.save_directory)? {
                    Some(playlist) => playlist,
                    None => Playlist::new(playlist_title),
                }
            }
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
        _ => playlist.save_playlist(&config.save_directory)?,
    }

    Ok(())
}
