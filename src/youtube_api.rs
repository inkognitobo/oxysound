//! Handles calls to the YouTube API

use crate::config::Config;
use crate::prelude::*;
use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Data structure for snippet
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSnippet {
    pub published_at: Option<String>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub channel_title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub category_id: Option<String>,
}

/// Data structure for a response item
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseItem {
    pub kind: String,
    pub id: String,
    pub snippet: ResponseSnippet,
}

/// Data structure for API responses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub kind: String,
    pub items: Vec<ResponseItem>,
}

fn create_videos_request(video_ids: &[String]) -> Result<String> {
    const API_URL: &str = "https://youtube.googleapis.com/youtube/v3/videos?part=snippet%2CcontentDetails%2Cstatistics";
    let config: Config = confy::load("oxysound", "config")?;
    let api_key = config.youtube_api_key;
    let key_url = format!("&key={}", api_key);

    let id_url = format!("&id={}", video_ids.join(","));

    Ok(format!("{}{}{}", API_URL, id_url, key_url))
}

pub async fn make_video_request(video_ids: &[String]) -> Result<Response> {
    let url = create_videos_request(video_ids)?;
    let client = Client::new();
    let response = client
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rick_astley_response() -> Response {
        Response {
            kind: "youtube#videoListResponse".into(),
            items: vec![ResponseItem {
                kind: "youtube#video".into(),
                id: "dQw4w9WgXcQ".into(),
                snippet: ResponseSnippet {
                    published_at: Some("2009-10-25T06:57:33Z".into()),
                    channel_id: Some("UCuAXFkgsw1L7xaCfnd5JJOw".into()),
                    title: Some(
                        "Rick Astley - Never Gonna Give You Up (Official Music Video)".into(),
                    ),
                    description: Some(
                        "The official video for “Never Gonna Give You Up” by Rick Astley".into(),
                    ),
                    channel_title: Some("Rick Astley".into()),
                    tags: Some(vec!["".into()]),
                    category_id: Some("10".into()),
                },
            }],
        }
    }

    #[test]
    #[ignore = "test requires API key"]
    fn test_create_video_request() {
        let request = create_videos_request(&["dQw4w9WgXcQ".into()])
            .expect("Expect config to load successfully for test");
        assert!(request.contains("https://youtube.googleapis.com/youtube/v3/videos?part=snippet%2CcontentDetails%2Cstatistics"));
        assert!(request.contains("&id=dQw4w9WgXcQ"));

        let request = create_videos_request(&["dQw4w9WgXcQ".into(), "y6120QOlsfU".into()])
            .expect("Expect config to load successfully for test");
        assert!(request.contains("https://youtube.googleapis.com/youtube/v3/videos?part=snippet%2CcontentDetails%2Cstatistics"));
        assert!(request.contains("&id=dQw4w9WgXcQ,y6120QOlsfU"));
    }

    #[tokio::test]
    #[ignore = "test requires API key"]
    async fn test_make_video_request() {
        let expected_response = rick_astley_response();
        let expected_first_item = expected_response
            .items
            .first()
            .expect("Has exactly one item");

        let response = make_video_request(&["dQw4w9WgXcQ".into()])
            .await
            .expect("Expect request to succeed for testing purposes");
        let first_item = response
            .items
            .first()
            .expect("Should have first item if request succeeded");

        assert_eq!(response.kind, expected_response.kind);
        assert!(!response.items.is_empty());
        assert_eq!(first_item.id, expected_first_item.id);
        assert_eq!(first_item.kind, expected_first_item.kind);
        assert_eq!(
            first_item.snippet.published_at,
            expected_first_item.snippet.published_at
        );
        assert_eq!(
            first_item.snippet.channel_id,
            expected_first_item.snippet.channel_id
        );
        assert_eq!(first_item.snippet.title, expected_first_item.snippet.title);
        assert_eq!(
            first_item.snippet.channel_title,
            expected_first_item.snippet.channel_title
        );
        assert_eq!(
            first_item.snippet.category_id,
            expected_first_item.snippet.category_id
        );
    }
}
