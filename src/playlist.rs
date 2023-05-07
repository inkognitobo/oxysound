/// Compose the playlist url using the base url and a list of video IDs
/// * ids - contains a list of valid YouTube video identifiers
pub fn compose_playlist_url(ids: Vec<String>) -> String {
    const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

    let mut url = String::from(BASE_URL);
    url.push_str(&ids.join(","));
    url
}
