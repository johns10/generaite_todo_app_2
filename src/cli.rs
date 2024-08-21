use clap::{Command, ArgMatches};
use crate::config::Config;
use crate::cli::db_commands::create_database;

mod db_commands;

/// Builds the command-line interface for the application.
pub fn build_cli() -> Command {
    Command::new("gen_todo")
        .about("A Todo application")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("version")
                .about("Prints the application version")
        )
        .subcommand(
            Command::new("create-db")
                .about("Creates the database based on the configuration")
        )
}

/// Runs the CLI logic based on the provided arguments and configuration.
pub fn run_cli(matches: &ArgMatches, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("gen_todo version {}", config.version);
            Ok(())
        }
        Some(("create-db", _)) => {
            create_database(config)?;
            println!("Database created successfully");
            Ok(())
        }
        _ => unreachable!(), // clap ensures we don't get here
    }
}
