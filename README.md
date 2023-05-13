# oxysound

Rust powered command line tool to create account independent YouTube playlists by composing a URL using `http://www.youtube.com/watch_videos?video_ids=` and a comma separated list of YouTube video IDs.

Playlists can be created, modified and saved as `JSON` encoded files via the command line interface.

To simply compose a playlist URL of a list of video IDs use the `print` operation:
```
oxysound print --ids <IDS>...>
```

All other operations will request video meta data (such as title, etc.) via YouTube's API and thus require a valid API key.

The API key and save directory can be configured via a `config.toml` file. 
When running e.g. `oxysound print --ids dQw4w9WgXcQ` for the first time, the application will ask the user to configure these values and inform them about the config file path.
For example on linux the config file will be located at `$HOME/.config/oxysound/config.toml`.

For more information run `oxysound --help`.
    
