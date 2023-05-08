/// Compose the playlist url using the base url and a list of video IDs
/// * ids - contains a list of valid YouTube video identifiers
pub fn compose_playlist_url(ids: Vec<String>) -> String {
    const BASE_URL: &str = "http://www.youtube.com/watch_videos?video_ids=";

    let mut url = String::from(BASE_URL);
    url.push_str(&ids.join(","));
    url
}

#[cfg(test)]
mod tests {
    use crate::playlist::compose_playlist_url;

    #[test]
    fn test_compose_playlist_url() {
        assert_eq!(
            compose_playlist_url(vec![
                String::from("test_id1"),
                String::from("test_id2"),
                String::from("test_id3")
            ]),
            String::from(
                "http://www.youtube.com/watch_videos?video_ids=test_id1,test_id2,test_id3"
            )
        );
    }
}
