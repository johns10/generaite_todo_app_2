mod cli;
mod config;

use crate::config::Config;
use env_logger::Env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::load()?;
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    cli::run_cli(&matches, &config)?;
    Ok(())
}
