use crate::error::Error;
use crate::prelude::*;
use crate::youtube_api::{self, ResponseItem};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::ErrorKind;

/// Data structure for video meta data
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
    title: String,
    published_at: String,
    url: String,
}

impl Video {
    /// Constructor to create a video instance with just the `id` field populated
    pub fn from_id(id: String) -> Self {
        let mut video = Video {
            id,
            title: String::from(""),
            published_at: String::from(""),
            url: String::from(""),
        };
        video.url = video.get_url();
        video
    }

    /// Concatenate video url using the video's ID
    fn get_url(&self) -> String {
        const BASE_URL: &str = "https://www.youtube.com/watch?v=";
        format!("{}{}", BASE_URL, self.id)
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<ResponseItem> for Video {
    fn from(value: ResponseItem) -> Self {
        let mut video = Video {
            id: value.id,
            title: value.snippet.title,
            published_at: value.snippet.published_at,
            url: String::from(""),
        };
        video.url = video.get_url();
        video
    }
}

/// Data structure for a playlist
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    name: String,
    num_items: u8,
    videos: Vec<Video>,
    url: String,
}

impl Playlist {
    /// Constructor
    pub fn new(name: &str, videos: Vec<Video>) -> Self {
        let mut playlist = Playlist {
            name: name.to_string(),
            num_items: videos.len() as u8,
            videos,
            url: String::from(""),
        };
        todo!("Use default");
        playlist.url = compose_playlist_url(&playlist.videos);
        playlist
    }

    /// Getter for url
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Add videos to the playlist
    ///
    /// New videos are added to `self.videos` (duplicates are ignored)
    /// `self.num_items` and `self.url` are updated accordingly
    /// * `videos` - list of `Video` data structure containing video meta data
    pub fn add_videos(&mut self, videos: &mut Vec<Video>) {
        videos.retain(|video| !self.videos.contains(video));

        self.num_items += videos.len() as u8;
        self.videos.append(videos);
        self.url = compose_playlist_url(&self.videos);
    }

    /// Use YouTube's API to accumulate video meta data in `self.videos`
    pub async fn fetch_metadata(&mut self) -> Result<()> {
        let ids: Vec<String> = self
            .videos
            .iter()
            .map(|video| video.id.to_string())
            .collect();
        if let Ok(response) = youtube_api::make_video_request(&ids).await {
            self.videos = response.items.into_iter().map(Video::from).collect();
        }
        Ok(())
    }
}

/// Return a `String` containing the playlist URL
///
/// Compose the playlist url using the base url and a comma separated list of video IDs
/// * `videos` - array of `Video`s that each contain valid YouTube IDs
pub fn compose_playlist_url(videos: &[Video]) -> String {
    const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

    let ids: Vec<String> = videos.iter().map(|video| video.id.to_string()).collect();

    format!("{}{}", &BASE_URL, &ids.join(","))
}

/// Serialize a `Playlist` instance and write content to a JSON file using the playlist's name as file name
/// * `playlist` - data structure containing playlist metadata
/// * `file_path` - path to the save directory
pub fn save_playlist(playlist: &Playlist, file_path: &str) -> Result<()> {
    let file_path = format!("{}{}{}", &file_path, &playlist.name, ".json");

    let playlist_json: String = serde_json::to_string(&playlist)?;
    fs::write(file_path, playlist_json)?;

    Ok(())
}

/// Return a `Playlist` instance.
///
/// Try to load content from a JSON file and deserialize into `Playlist` instance
/// If `load_or_create_file` returns `Ok(None)`, return an empty playlist named `playlist_name`
/// * `playlist_name` - name of the playlist which is used as file name
/// * `file_path` - path to the save directory
pub fn load_playlist(playlist_name: &str, file_path: &str) -> Result<Playlist> {
    let file_path = format!("{}{}{}", &file_path, &playlist_name, ".json");

    match load_or_create_file(&file_path)? {
        Some(playlist_json) => Ok(serde_json::from_str(&playlist_json)?),
        None => Ok(Playlist::new(playlist_name, vec![])),
    }
}

/// Return file content if file exists, else `None`
///
/// If the file exists, return its content as `Some<String>`
/// If the file does not exist, create it and return `None`
/// * `file_path` - full file path (e.g. "./test.json")
fn load_or_create_file(file_path: &str) -> Result<Option<String>> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(Some(content)),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                File::create(file_path)?;
                Ok(None)
            }
            _ => Err(Error::IO(error)),
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
                Video::from_id(String::from("test_id1")),
                Video::from_id(String::from("test_id2")),
                Video::from_id(String::from("test_id3")),
            ]),
            String::from(
                "http://www.youtube.com/watch_videos?video_ids=test_id1,test_id2,test_id3"
            )
        );
    }
}
