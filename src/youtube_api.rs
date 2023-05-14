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
