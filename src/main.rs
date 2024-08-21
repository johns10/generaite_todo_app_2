mod cli;
mod config;

use crate::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    cli::run_cli(&matches, &config)?;
    Ok(())
}
