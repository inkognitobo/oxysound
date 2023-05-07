mod args;
mod playlist;

use crate::args::Args;
use crate::playlist::compose_playlist_url;
use clap::Parser;

fn main() {
    let arguments = Args::parse();

    println!("{:?}", compose_playlist_url(arguments.ids));
}
