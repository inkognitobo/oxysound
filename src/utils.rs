//! Crate utils module

use crate::prelude::*;
use directories::BaseDirs;
use std::{ffi::OsStr, fs, path::PathBuf};

/// Return file content if file exists, else create the file
///
/// Try to read file content to `String`
/// If the file exists, return content
/// If the file doesn't exist, try to create it and return `None`
/// * `file_path` - full file path (e.g. "./test.json")
pub fn load_or_create_file(file_path: PathBuf) -> Result<Option<String>> {
    let file_path = expand_path_aliases(file_path);

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

/// Return a `PathBuf`that has common aliases in file paths replaced with the paths of user-invisible standard directories
///
/// If the operating system has known user-invisible directories:
/// Checks if a path's component can be converted to a `&str` (only contains valid Unicode)
/// yes -> pattern match and expand common aliases to full paths
/// no -> leave the component untouched
pub fn expand_path_aliases(file_path: PathBuf) -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        let file_path = file_path
            .iter()
            .map(|component| {
                if let Some(component_str) = component.to_str() {
                    match component_str {
                        "$HOME" => base_dirs.home_dir().as_os_str(),
                        "$XDG_CACHE_HOME" => base_dirs.cache_dir().as_os_str(),
                        "$XDG_CONFIG_HOME" => base_dirs.config_dir().as_os_str(),
                        "$XDG_DATA_HOME" => base_dirs.data_dir().as_os_str(),
                        "$XDG_BIN_HOME" => base_dirs
                            .executable_dir()
                            .expect("User should only use '$XDG_BIN_HOME' if on linux")
                            .as_os_str(),
                        _ => OsStr::new(component_str),
                    }
                } else {
                    component
                }
            })
            .collect();
        file_path
    } else {
        file_path
    }
}
