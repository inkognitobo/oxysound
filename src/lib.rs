//! Main crate logic

pub mod args;
pub mod config;
mod error;
mod playlist;
mod prelude;
mod utils;
mod youtube_api;

use std::fs::read_dir;
use std::path::PathBuf;

use crate::args::{Arguments, Operation};
use crate::playlist::Playlist;
use crate::prelude::*;

/// Run the application
///
/// * `args` - parsed CLI arguments
/// * `save_directory` - location to look for saved playlists and save playlist after applying changes
pub async fn run(args: Arguments, save_directory: impl Into<String>) -> Result<()> {
    let save_directory = save_directory.into();
    let mut save = false;
    let mut print_url = true;

    let playlist = match args.operation {
        Operation::Add(args) => {
            save = true;
            add(args.playlist_title, args.ids, &save_directory).await?
        }
        Operation::Remove(args) => {
            save = true;
            remove(args.playlist_title, args.ids, &save_directory)?
        }
        Operation::Print(args) => print(args.playlist_title, args.ids, &save_directory)?,
        Operation::List => {
            print_url = false;
            list(&save_directory)?;
            Playlist::default()
        }
    };

    // Print playlist URL depending on the selected operation
    if print_url {
        println!("{}", playlist);
    }

    // Save the playlist depending on the selected operation
    if save {
        playlist.save_playlist(&save_directory)?;
    }

    Ok(())
}

/// Add videos to playlist
///
/// If a file_path is provided, videos are added to existing playlist.
/// Otherwise a new playlist containing the videos is created.
///
/// * `playlist_title` - name of the playlist
/// * `ids` - list of video IDs
/// * `file_directory` - location to look for existing playlist or save new playlist
async fn add(
    playlist_title: String,
    ids: Vec<String>,
    file_path: impl Into<String>,
) -> Result<Playlist> {
    let mut playlist = match playlist::load_playlist(&playlist_title, file_path)? {
        Some(playlist) => playlist,
        None => Playlist::new(&playlist_title),
    };
    playlist.add_videos(&ids);
    playlist.fetch_metadata().await?;

    Ok(playlist)
}

/// Remove videos from playlist
///
/// If a file_path is provided, videos are removed from existing playlist.
/// Otherwise a new playlist containing the videos is created.
/// The latter operation functionally does nothing and merely allows for simplifications.
///
/// * `playlist_title` - name of the playlist
/// * `ids` - list of video IDs
/// * `file_directory` - location to look for existing playlist or save new playlist
fn remove(
    playlist_title: String,
    ids: Vec<String>,
    file_path: impl Into<String>,
) -> Result<Playlist> {
    let mut playlist = match playlist::load_playlist(&playlist_title, file_path)? {
        Some(playlist) => playlist,
        None => Playlist::new(&playlist_title),
    };
    playlist.remove_videos(&ids);
    Ok(playlist)
}

/// Print playlist URL to `stdout`
///
/// If a file_path is provided, existing playlist is used.
/// Otherwise a new playlist containing the videos is used.
/// These arguments have to be mutually exclusive.
///
/// * `playlist_title` - name of the playlist
/// * `ids` - list of video IDs
/// * `file_directory` - location to look for existing playlist or save new playlist
fn print(
    playlist_title: Option<String>,
    ids: Option<Vec<String>>,
    file_path: impl Into<String>,
) -> Result<Playlist> {
    match (playlist_title, ids) {
        (Some(playlist_title), None) => {
            match playlist::load_playlist(&playlist_title, file_path)? {
                Some(playlist) => Ok(playlist),
                None => Ok(Playlist::new(&playlist_title)),
            }
        }
        (None, Some(ids)) => {
            let mut playlist = Playlist::default();
            playlist.add_videos(&ids);
            Ok(playlist)
        }
        _ => unreachable!(),
        // Unreachable because `PrintArgs.playlist_title` and `PrintArgs.ids` are mutually exclusive
    }
}

/// Print a list of all available playlists
///
/// * `file_directory` - location to look for playlists
fn list(file_path: impl Into<String>) -> Result<()> {
    let file_path = file_path.into();
    let mut file_path: PathBuf = PathBuf::from(&file_path);

    file_path = utils::expand_path_aliases(file_path);

    println!("Available playlists at {:?}:", &file_path);

    for entry in read_dir(&file_path)?.filter_map(|entry| entry.ok()) {
        let entry = entry.path();
        let entry = entry
            .file_stem()
            .ok_or_else(|| Error::StringFromPathBuf(format!("{:?}", entry)))?
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::StringFromPathBuf(format!("{:?}", entry)))?;

        println!("- {}", entry);
    }

    Ok(())
}
