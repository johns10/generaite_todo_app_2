mod cli;
mod config;
mod web_server;

use crate::config::Config;
use crate::web_server::{AxumWebServer, WebServerContext};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    cli::run_cli(&matches, &config)?;

    // Create and run the web server
    let server = AxumWebServer::new(&config);
    let context = WebServerContext::new(server);
    context.run().await?;

    Ok(())
}
