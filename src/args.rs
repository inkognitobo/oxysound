use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = false)]
    pub ids: Vec<String>,
    /// Name of the playlist
    #[arg(short, long, required = false)]
    pub playlist_name: Option<String>,
}
