use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List of video IDs
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub ids: Vec<String>,
}
