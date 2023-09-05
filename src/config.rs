//! Handles everything related to the program's config file

use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // E.g. "KiasdlLLkgUUIOOsd-7ASGkdskgT9ka9JlsdgkP" <- just an example key
    pub youtube_api_key: String,
    // E.g. "$XDG_DATA_HOME/oxysound/playlists"
    pub save_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            youtube_api_key: "".into(),
            save_directory: "$XDG_DATA_HOME/oxysound/playlists".into(),
        }
    }
}

impl Config {
    /// Return `self` after ensuring that all required values are configured properly
    /// Else return an Error
    pub fn assert_values(self) -> Result<Self> {
        let config_file_path = confy::get_configuration_file_path("oxysound", "config")?;
        let config_file_path = config_file_path
            .to_str()
            // This should realistically never happen
            // Suggests that the user specified a user name that contains invalid Unicode
            // Hence e.g. `home/USER_NAME/.config` contains invalid Unicode
            .expect("Path contains non-UTF-8 strings")
            .to_string();
        if self.youtube_api_key.is_empty() {
            return Err(Error::MissingConfig(
                "youtube_api_key".to_string(),
                config_file_path,
            ));
        }
        if self.save_directory.is_empty() {
            return Err(Error::MissingConfig(
                "save_directory".to_string(),
                config_file_path,
            ));
        }
        Ok(self)
    }
}
