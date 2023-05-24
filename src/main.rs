use clap::Parser;
use oxysound::{args::Arguments, config::Config};
use std::process;

#[tokio::main]
async fn main() {
    let config: Config = match confy::load::<Config>("oxysound", "config") {
        Ok(config) => match config.assert_values() {
            Ok(config) => config,
            Err(e) => {
                eprint!("Config error: {e}");
                process::exit(1);
            }
        },
        Err(e) => {
            eprint!("Config error: {e}");
            process::exit(1);
        }
    };
    let args = Arguments::parse();

    if let Err(e) = oxysound::run(args, &config.save_directory).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
