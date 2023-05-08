use serde::{Deserialize, Serialize};
use std::fs;

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

    pub fn videos(&self) -> &Vec<Video> {
        &self.videos
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
/// * videos - array of `Video`s that each contain valid YouTube IDs
pub fn compose_playlist_url(videos: &[Video]) -> String {
    const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

    let ids: Vec<String> = videos.iter().map(|video| video.id.to_string()).collect();

    let mut url = String::from(BASE_URL);
    url.push_str(&ids.join(","));
    url
}

/// Serialize a `Playlist` and write content to a .json file using the playlist's name as file name
/// * playlist - data structure containing playlist metadata
/// * file_path - path to the save directory
pub fn write_to_file(playlist: &Playlist, file_path: &str) -> std::io::Result<()> {
    let mut file_path = file_path.to_string();
    file_path.push_str(&playlist.name);
    file_path.push_str(".json");

    let playlist_json: String = serde_json::to_string(&playlist)
        .expect("Should be able to convert Playlist to json string");

    fs::write(file_path, &playlist_json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::playlist::{compose_playlist_url, Playlist, Video};

    #[test]
    fn test_video_object() {
        Video::new(String::from("test_id"));
    }

    #[test]
    fn test_playlist_object() {
        Playlist::new("Test", vec![Video::new(String::from("test_id"))]);
    }

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
