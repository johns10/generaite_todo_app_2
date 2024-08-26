mod cli;
mod config;
mod web_server;

use crate::config::Config;
use crate::services::web_server::{WebServerStrategy, AxumWebServer, WebServerContext};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load()?;
    let cli = cli::build_cli();
    let matches = cli.get_matches();

    // Handle CLI commands
    cli::run_cli(&matches, &config)?;

    // Check if we should start the web server
    if should_start_web_server(&matches) {
        info!("Starting web server");
        run_web_server(&config).await?;
    } else {
        info!("Web server not started (use --server flag to start)");
    }

    Ok(())
}

fn should_start_web_server(matches: &clap::ArgMatches) -> bool {
    matches.contains_id("server")
}

async fn run_web_server(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let server = AxumWebServer::new(config);
    let context = WebServerContext::new(server);
    context.run().await.map_err(|e| {
        error!("Web server error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })
}
