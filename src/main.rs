mod args;
mod error;
mod playlist;
mod youtube_api;

use crate::args::Args;
use crate::playlist::{load_playlist, Playlist, Video};
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let file_path: String = String::from("./");

    let arguments = Args::parse();

    let mut playlist = match &arguments.playlist_name {
        // If no playlist name provided, create an temporary empty one
        None => Playlist::new("tmp", vec![]),
        // Otherwise try to load the specified playlist
        Some(playlist_name) => match load_playlist(playlist_name, &file_path) {
            Ok(playlist) => playlist,
            Err(error) => panic!("There was a problem loading the playlist: {:?}", error),
        },
    };

    let mut videos: Vec<Video> = arguments
        .ids
        .iter()
        .map(|id| Video::from_id(id.to_string()))
        .collect();

    playlist.add_videos(&mut videos);

    playlist.fetch_metadata().await;

    match &arguments.playlist_name {
        None => (),
        Some(_) => {
            playlist::save_playlist(&playlist, "./");
        }
    }

    println!("Playlist URL:\n{:?}", playlist.url());
    Ok(())
}
