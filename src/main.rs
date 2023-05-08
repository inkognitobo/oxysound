mod args;
mod playlist;

use crate::args::Args;
use crate::playlist::{Playlist, Video};
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Args::parse();

    let videos: Vec<Video> = arguments
        .ids
        .iter()
        .map(|id| Video::new(id.to_string()))
        .collect();

    let playlist = Playlist::new("test", videos);

    println!("{:?}", playlist::compose_playlist_url(playlist.videos()));

    playlist::write_to_file(&playlist, "./")?;
    Ok(())
}
