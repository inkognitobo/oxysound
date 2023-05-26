//! Main crate logic

use crate::error::Error;
use crate::youtube_api::{self, ResponseItem};
use crate::{prelude::*, utils};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::{self};
use std::path::PathBuf;

/// Data structure for video meta data
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    id: String,
    title: String,
    published_at: String,
    url: String,
    fetched: bool,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            id: "".into(),
            title: "".into(),
            published_at: "".into(),
            url: "https://www.youtube.com/watch?v=".into(),
            fetched: false,
        }
    }
}

impl From<String> for Video {
    fn from(value: String) -> Self {
        let mut video = Self {
            id: value,
            ..Default::default()
        };
        video.update_fields();
        video
    }
}

impl From<ResponseItem> for Video {
    fn from(value: ResponseItem) -> Self {
        let mut video = Self {
            id: value.id,
            title: value.snippet.title.unwrap_or_default(),
            published_at: value.snippet.published_at.unwrap_or_default(),
            fetched: true,
            ..Default::default()
        };
        video.update_fields();
        video
    }
}

impl PartialEq for Video {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Video {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_and_time = self.published_at.split("T").collect::<Vec<&str>>();
        let date = date_and_time.first().unwrap_or(&"unknown date");

        return write!(
            f,
            "{}\n\tID: {}\n\tPublished at: {}\n\tURL: {}",
            self.title, self.id, date, self.url
        );
    }
}

impl Video {
    /// Update fields that depend on other fields
    /// e.g. `self.url` depends on `self.id`
    fn update_fields(&mut self) {
        const BASE_URL: &str = "https://www.youtube.com/watch?v=";
        self.url = format!("{}{}", BASE_URL, self.id);
    }
}

/// Data structure for a playlist
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    title: String,
    num_items: u8,
    videos: Vec<Video>,
    url: String,
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            title: "untitled".into(),
            num_items: 0,
            videos: Vec::new(),
            url: "http://www.youtube.com/watch_videos?video_ids=".into(),
        }
    }
}

impl Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let videos = self
            .videos
            .iter()
            .map(|video| format!("{}", video).replace("\t", "\t\t"))
            .map(|video_string| format!("\t{}", video_string))
            .collect::<Vec<String>>()
            .join("\n\n");
        write!(
            f,
            "{}\n----------\nlength: {}\nvideos: \n{}\n\nplaylist URL: {}",
            self.title, self.num_items, videos, self.url
        )
    }
}

impl Playlist {
    pub fn new(title: impl Into<String>) -> Self {
        let mut playlist = Self {
            title: title.into(),
            ..Default::default()
        };
        playlist.update_fields();
        playlist
    }

    /// Update fields that depend on other fields
    /// e.g. `self.num_items` depends on `self.videos`
    fn update_fields(&mut self) {
        self.num_items = self.videos.len() as u8;
        self.url = self.compose_playlist_url();
    }

    /// Add videos to the playlist
    ///
    /// New videos are added to `self.videos` (duplicates are ignored)
    /// Fields are then updated accordingly
    /// * `ids` - list of video IDs
    pub fn add_videos(&mut self, ids: &[String]) {
        let mut videos = ids
            .iter()
            .map(|id| id.to_string())
            .map(Video::from)
            .collect::<Vec<Video>>();
        videos.retain(|video| !self.videos.contains(video));

        self.videos.append(&mut videos);
        self.update_fields();
    }

    /// Remove videos from the playlist
    ///
    /// If an invalid ID is provided (not in playlist), simply nothing happens
    /// * `ids` - list of video IDs
    pub fn remove_videos(&mut self, ids: &[String]) {
        self.videos.retain(|video| !ids.contains(&video.id));
        self.update_fields();
    }

    /// Return a `String` containing the playlist URL
    ///
    /// The URL is composed using the base url and a comma separated list of video IDs
    fn compose_playlist_url(&self) -> String {
        const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

        let ids: Vec<String> = self
            .videos
            .iter()
            .map(|video| video.id.to_string())
            .collect();

        format!("{}{}", &BASE_URL, &ids.join(","))
    }

    /// Use YouTube's API to accumulate video meta data in `self.videos`
    /// Only request data for videos, that has no attached meta data yet
    pub async fn fetch_metadata(&mut self) -> Result<()> {
        let ids: Vec<String> = self
            .videos
            .iter()
            .filter(|video| !video.fetched)
            .map(|video| video.id.to_string())
            .collect();

        let response = youtube_api::make_video_request(&ids).await?;
        let mut newly_fetched = response
            .items
            .into_iter()
            .map(Video::from)
            .collect::<Vec<Video>>();

        let num_requested = ids.len();
        let num_fetched = newly_fetched.len();

        if num_fetched == num_requested {
            self.videos.retain(|video| video.fetched);
            self.videos.append(&mut newly_fetched);
            Ok(())
        } else {
            Err(Error::NotEnoughResponseItems(
                num_requested as u8,
                num_fetched as u8,
            ))
        }
    }

    /// Serialize a `Playlist` instance and write content to a JSON file using the playlist's title as file name
    /// * `file_path` - path to the save directory
    pub fn save_playlist(&self, file_path: impl Into<String>) -> Result<()> {
        let file_path = file_path.into();
        let mut file_path: PathBuf = [&file_path, &self.title].iter().collect();
        file_path.set_extension("json");

        file_path = utils::expand_path_aliases(file_path);

        let playlist_json: String = serde_json::to_string(self)?;
        fs::write(file_path, playlist_json)?;

        Ok(())
    }
}

/// Return a `Playlist` instance.
///
/// Try to load content from a JSON file and deserialize into `Playlist` instance
/// * `playlist_title` - name of the playlist
/// * `file_path` - path to the save directory
pub fn load_playlist(
    playlist_title: impl Into<String>,
    file_path: impl Into<String>,
) -> Result<Option<Playlist>> {
    let playlist_title = playlist_title.into();
    let file_path = file_path.into();
    let mut file_path: PathBuf = [&file_path, &playlist_title].iter().collect();
    file_path.set_extension("json");

    file_path = utils::expand_path_aliases(file_path);

    match load_or_create_file(file_path)? {
        None => {
            println!(
                "Playlist {0} does not exist, creating {0} instead",
                &playlist_title
            );
            Ok(None)
        }
        Some(playlist_json) => {
            let playlist = serde_json::from_str(&playlist_json)?;
            Ok(playlist)
        }
    }
}

/// Return file content if file exists, else create the file
///
/// Try to read file content to `String`
/// If the file exists, return content
/// If the file doesn't exist, try to create it and return `None`
/// * `file_path` - full file path (e.g. "./test.json")
fn load_or_create_file(file_path: PathBuf) -> Result<Option<String>> {
    let file_path = utils::expand_path_aliases(file_path);

    match fs::read_to_string(&file_path) {
        Ok(content) => Ok(Some(content)),
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                fs::File::create(&file_path)?;
                Ok(None)
            }
            _ => Err(Error::from(error)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video() {
        assert_eq!(
            Video::from("id_1".to_string()),
            Video {
                id: "id_1".into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_fields_video() {
        let mut video = Video {
            id: "id_1".into(),
            ..Default::default()
        };
        video.update_fields();

        assert_eq!(
            video.url,
            "https://www.youtube.com/watch?v=id_1".to_string()
        );
        assert_eq!(
            video,
            Video {
                id: "id_1".into(),
                url: "https://www.youtube.com/watch?v=id_1".into(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_playlist() {
        let mut playlist_1 = Playlist {
            title: "test".into(),
            ..Default::default()
        };
        playlist_1.update_fields();
        assert_eq!(Playlist::new("test",), playlist_1);

        let mut playlist_2 = Playlist {
            title: "test".into(),
            videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
            ..Default::default()
        };
        playlist_2.update_fields();
        assert_eq!(
            Playlist {
                title: "test".into(),
                videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
                num_items: 2,
                url: "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".into()
            },
            playlist_2
        );
    }

    #[test]
    fn test_add_videos() {
        let mut playlist = Playlist {
            title: "test".into(),
            ..Default::default()
        };
        playlist.add_videos(&["id_1".into(), "id_2".into()]);

        assert_eq!(
            playlist,
            Playlist {
                title: "test".into(),
                num_items: 2,
                videos: vec![
                    Video {
                        id: "id_1".into(),
                        ..Default::default()
                    },
                    Video {
                        id: "id_2".into(),
                        ..Default::default()
                    }
                ],
                url: "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".into()
            }
        );
    }

    #[test]
    fn test_remove_videos() {
        let mut playlist = Playlist {
            title: "test".into(),
            num_items: 2,
            videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
            url: "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".into(),
        };
        playlist.remove_videos(&["id_1".into(), "id_2".into()]);

        assert_eq!(
            playlist,
            Playlist {
                title: "test".into(),
                num_items: 0,
                videos: vec![],
                url: "http://www.youtube.com/watch_videos?video_ids=".into()
            }
        );
    }

    #[test]
    fn test_compose_url() {
        let playlist = Playlist {
            title: "test".into(),
            videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
            ..Default::default()
        };

        assert_eq!(
            playlist.compose_playlist_url(),
            "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".to_string()
        );
    }

    #[test]
    fn test_update_fields_playlist() {
        let mut playlist = Playlist {
            title: "test".into(),
            videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
            ..Default::default()
        };
        playlist.update_fields();

        assert_eq!(playlist.num_items, 2);
        assert_eq!(
            playlist.url,
            "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".to_string()
        );
        assert_eq!(
            playlist,
            Playlist {
                title: "test".into(),
                num_items: 2,
                videos: vec![
                    Video {
                        id: "id_1".into(),
                        ..Default::default()
                    },
                    Video {
                        id: "id_2".into(),
                        ..Default::default()
                    },
                ],
                url: "http://www.youtube.com/watch_videos?video_ids=id_1,id_2".into()
            }
        );
    }
    #[tokio::test]
    #[ignore = "test requires API key"]
    async fn test_fetch_metadata() -> Result<()> {
        let mut playlist = Playlist {
            title: "test".into(),
            videos: vec!["dQw4w9WgXcQ".to_string().into()],
            num_items: 1,
            url: "http://www.youtube.com/watch_videos?video_ids=dQw4w9WgXcQ".into(),
        };
        playlist.fetch_metadata().await?;

        assert_eq!(
            playlist
                .videos
                .iter()
                .filter(|video| !video.fetched)
                .count(),
            0
        );
        assert_eq!(
            playlist
                .videos
                .iter()
                .next()
                .expect("Test playlist should have one video")
                .title,
            "Rick Astley - Never Gonna Give You Up (Official Music Video)"
        );

        Ok(())
    }
}
