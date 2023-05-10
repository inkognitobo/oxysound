mod args;
mod error;
mod playlist;
mod prelude;
mod youtube_api;

use crate::args::Args;
use crate::playlist::{load_playlist, Playlist};
use crate::prelude::*;
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let file_path: String = String::from("./");

    let mut arguments = Args::parse();

    let mut playlist = match &arguments.playlist_name {
        // If no playlist name provided, create an temporary empty one
        None => Playlist::new("tmp"),
        // Otherwise try to load the specified playlist
        Some(playlist_name) => match load_playlist(playlist_name, &file_path) {
            Ok(playlist) => playlist,
            Err(error) => panic!("There was a problem loading the playlist: {:?}", error),
        },
    };

    playlist.add_videos(&mut arguments.ids);

    playlist.fetch_metadata().await?;

    // If there was a name specified, the playlist is to be saved
    match &arguments.playlist_name {
        None => (),
        Some(_) => {
            playlist.save_playlist("./")?;
        }
    }

    println!("Playlist URL:\n{:?}", playlist.url());
    Ok(())
}
