# oxysound

![build](https://github.com/inkognitobo/oxysound/actions/workflows/rust_build.yml/badge.svg)
![test](https://github.com/inkognitobo/oxysound/actions/workflows/rust_test.yml/badge.svg)

<p align="center">
🎵 Rust powered command line tool to create and manage account independent YouTube playlists. 🎵
</p>

## 💡 Features

- Quickly generate a playlist URL of a list of video IDs using `oxysound print --ids <IDS>...`.
- Create and manage YouTube playlists and save them as `.json` files.
- Fetching video meta data via YouTube's API

## 🔌 Installation

Download the latest [release](https://github.com/inkognitobo/oxysound/releases/latest) or build from source:
```
git clone https://github.com/inkognitobo/oxysound.git
cd oxysound
cargo build --release
```

## 🛠️ Setup

To utilise all features obtain an API key for [YouTube's Data API](https://console.cloud.google.com/apis/library/youtube.googleapis.com).
See [Configuration](https://github.com/inkognitobo/oxysound?tab=readme-ov-file#%EF%B8%8F-configuration) for more information.

## ⚙️ Configuration

The API key and save directory can be configured via a `config.toml` file. 
When running a command like `oxysound --help` for the first time, the application will ask the user to configure these values and inform them about the config file path.
For example on Linux the config file will be located at `$HOME/.config/oxysound/config.toml`.

For more information run `oxysound --help`.
