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
    Add(ModifyArgs),
    /// Remove videos from existing playlist
    Remove(ModifyArgs),
    /// Print playlist URL of an existing playlist or list of IDs
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Title of the playlist
    #[arg(short = 't', long, required = true)]
    pub playlist_title: String,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = false)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct ModifyArgs {
    /// Title of the playlist
    #[arg(short = 't', long, required = true)]
    pub playlist_title: String,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = true)]
    pub ids: Vec<String>,
}

#[derive(Debug, Args)]
#[group(multiple = false, required = true)]
pub struct PrintArgs {
    /// Title of the playlist
    #[arg(short = 't', long, required = false)]
    pub playlist_title: Option<String>,
    /// Space separated list of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ', required = false)]
    pub ids: Option<Vec<String>>,
}
