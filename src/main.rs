mod args;
mod error;
mod playlist;

use crate::args::Args;
use crate::playlist::{load_playlist, Playlist, Video};
use clap::Parser;

fn main() {
    let file_path: String = String::from("./");

    let arguments = Args::parse();

    let mut playlist = match &arguments.playlist_name {
        // If no playlist name provided, create an temporary empty one
        None => Playlist::new("tmp", vec![]),
        // Otherwise try to load the specified playlist
        Some(playlist_name) => match load_playlist(&playlist_name, &file_path) {
            Ok(playlist) => playlist,
            Err(error) => panic!("There was a problem loading the playlist: {:?}", error),
        },
    };

    let mut videos: Vec<Video> = arguments
        .ids
        .iter()
        .map(|id| Video::new(id.to_string()))
        .collect();

    playlist.add_videos(&mut videos);

    match &arguments.playlist_name {
        None => (),
        Some(_) => {
            playlist::save_playlist(&playlist, "./");
        }
    }

    println!(
        "Playlist URL:\n{:?}",
        playlist::compose_playlist_url(playlist.videos())
    );
}
