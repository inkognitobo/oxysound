use crate::error::Error;
use crate::prelude::*;
use crate::youtube_api::{self, ResponseItem};
use serde::{Deserialize, Serialize};
use std::fs::{self};

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
            id: value.into(),
            ..Default::default()
        };
        video.update_fields();
        video
    }
}

impl From<ResponseItem> for Video {
    fn from(value: ResponseItem) -> Self {
        let mut video = Self {
            id: value.id.into(),
            title: value.snippet.title.into(),
            published_at: value.snippet.published_at.into(),
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

impl Video {
    pub fn new(id: impl Into<String>) -> Self {
        let mut video = Self {
            id: id.into(),
            ..Default::default()
        };
        video.update_fields();
        video
    }

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

impl Playlist {
    pub fn new(title: impl Into<String>) -> Self {
        let mut playlist = Self {
            title: title.into(),
            ..Default::default()
        };
        playlist.update_fields();
        playlist
    }

    pub fn new_with_videos(title: impl Into<String>, videos: Vec<Video>) -> Self {
        let mut playlist = Self {
            title: title.into(),
            videos,
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
        let num_requested = ids.len();
        let response = youtube_api::make_video_request(&ids).await?;

        let mut newly_fetched = response
            .items
            .into_iter()
            .map(Video::from)
            .collect::<Vec<Video>>();
        let num_fetched = newly_fetched.len();

        if newly_fetched.len() == num_requested {
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
        let file_path = format!("{}{}{}", &file_path, self.title, ".json");

        let playlist_json: String = serde_json::to_string(self)?;
        fs::write(file_path, playlist_json)?;

        Ok(())
    }
}

/// Getter/setter functions
impl Playlist {
    /// URL
    pub fn url(&self) -> &str {
        &self.url
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
) -> Result<Playlist> {
    let playlist_title = playlist_title.into();
    let file_path = file_path.into();
    let file_path = format!("{}{}{}", file_path, playlist_title, ".json");

    let playlist_json = load_file(&file_path)?;
    let playlist = serde_json::from_str(&playlist_json)?;
    Ok(playlist)
}

/// Return file content if file exists
///
/// Try to read file content to string
/// * `file_path` - full file path (e.g. "./test.json")
fn load_file(file_path: impl Into<String>) -> Result<String> {
    let file_path = file_path.into();
    let content = fs::read_to_string(&file_path)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_video() {
        assert_eq!(
            Video::new("id_1"),
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
        assert_eq!(
            Playlist::new("test",),
            Playlist {
                title: "test".into(),
                ..Default::default()
            }
        );
        assert_eq!(
            Playlist::new_with_videos(
                "test",
                vec!["id_1".to_string().into(), "id_2".to_string().into()]
            ),
            Playlist {
                title: "test".into(),
                videos: vec!["id_1".to_string().into(), "id_2".to_string().into()],
                ..Default::default()
            }
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
    async fn test_fetch_metadata() -> Result<()> {
        dotenv().ok();

        let mut playlist =
            Playlist::new_with_videos("test", vec!["dQw4w9WgXcQ".to_string().into()]);
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
