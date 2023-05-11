use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Operation to perform
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Debug, Subcommand)]
pub enum Operation {
    /// Create new playlist
    Create(CreateArgs),
    /// Add videos to existing playlist
    Add(AddArgs),
    /// Remove videos from existing playlist
    Remove(RemoveArgs),
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Title of the playlist
    #[arg(short, long, required = true)]
    pub playlist_title: String,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = false)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Title of the playlist
    #[arg(short, long, required = true)]
    pub playlist_title: String,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true)]
    pub ids: Vec<String>,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Title of the playlist
    #[arg(short, long, required = true)]
    pub playlist_title: String,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true)]
    pub ids: Vec<String>,
}
