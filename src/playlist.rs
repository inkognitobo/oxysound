use crate::error::ReadPlaylistError;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::ErrorKind;

/// Data structure for a playlist
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    name: String,
    videos: Vec<Video>,
}

impl Playlist {
    /// Constructor
    pub fn new(name: &str, videos: Vec<Video>) -> Self {
        let name = name.to_string();

        Playlist { name, videos }
    }

    /// Getter
    pub fn videos(&self) -> &Vec<Video> {
        &self.videos
    }

    /// Add videos to the playlist
    /// * `videos` - list of `Video` data structure containing video meta data
    pub fn add_videos(&mut self, videos: &mut Vec<Video>) {
        self.videos.append(videos)
    }
}

/// Data structure for video meta data
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
}

impl Video {
    /// Constructor
    pub fn new(id: String) -> Self {
        Video { id }
    }
}

/// Compose the playlist url using the base url and a list of video IDs
/// * `videos` - array of `Video`s that each contain valid YouTube IDs
pub fn compose_playlist_url(videos: &[Video]) -> String {
    const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

    let ids: Vec<String> = videos.iter().map(|video| video.id.to_string()).collect();

    let mut url = String::from(BASE_URL);
    url.push_str(&ids.join(","));
    url
}

/// Serialize a `Playlist` instance and write content to a JSON file using the playlist's name as file name
/// * `playlist` - data structure containing playlist metadata
/// * `file_path` - path to the save directory
pub fn save_playlist(playlist: &Playlist, file_path: &str) {
    let file_path = format!("{}{}{}", &file_path, &playlist.name, ".json");

    let playlist_json: String =
        serde_json::to_string(&playlist).expect("TODO return custom error type");

    fs::write(file_path, &playlist_json).expect("TODO return custom error type");
}

/// Try to load content from a JSON file and deserialize into `Playlist` instance
/// If `load_or_create_file` returns Ok(None), return an empty playlist named `playlist_name`
/// * `playlist_name` - name of the playlist which is used as file name
/// * `file_path` - path to the save directory
pub fn load_playlist(playlist_name: &str, file_path: &str) -> Result<Playlist, ReadPlaylistError> {
    let file_path = format!("{}{}{}", &file_path, &playlist_name, ".json");

    match load_or_create_file(&file_path) {
        Ok(playlist_json_option) => match playlist_json_option {
            Some(playlist_json) => match serde_json::from_str(&playlist_json) {
                Ok(playlist) => Ok(playlist),
                Err(error) => Err(ReadPlaylistError::DeserializeError(error)),
            },
            None => Ok(Playlist::new(&playlist_name, vec![])),
        },
        Err(error) => Err(error),
    }
}

/// Try to load content from file
///
/// If the file exists, return its content as Some<String>
/// If the file does not exist, create it and return None
/// * `file_path` - full file path (e.g. "./test.json")
fn load_or_create_file(file_path: &str) -> Result<Option<String>, ReadPlaylistError> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(Some(content)),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(_) => Ok(None),
                Err(error) => Err(ReadPlaylistError::IOError(error)),
            },
            other_error => Err(ReadPlaylistError::IOError(other_error.into())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_playlist_url() {
        assert_eq!(
            compose_playlist_url(&[
                Video::new(String::from("test_id1")),
                Video::new(String::from("test_id2")),
                Video::new(String::from("test_id3")),
            ]),
            String::from(
                "http://www.youtube.com/watch_videos?video_ids=test_id1,test_id2,test_id3"
            )
        );
    }
}
