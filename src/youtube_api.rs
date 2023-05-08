use reqwest::header;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Data structure for API responses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub kind: String,
    pub items: Vec<ResponseItem>,
}

/// Data structure for a response item
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseItem {
    pub kind: String,
    pub id: String,
    pub snippet: ResponseSnippet,
}

/// Data structure for snippet
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSnippet {
    pub published_at: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub tags: Vec<String>,
    pub category_id: String,
}

pub async fn make_request(url: &str) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let response: Response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
