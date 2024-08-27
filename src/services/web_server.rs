use async_trait::async_trait;
use axum::Router;
use std::net::SocketAddr;
use thiserror::Error;
use tracing::{info, error};
use tokio::net::TcpListener;

use crate::config::Config;
use crate::services::database::DatabasePool;

/// Represents errors that can occur in the web server.
#[derive(Error, Debug)]
pub enum WebServerError {
    /// Represents a general server error with a message.
    #[error("Server error: {0}")]
    ServerError(String),
}

/// Defines the interface for web server strategies.
#[async_trait]
pub trait WebServerStrategy {
    /// Runs the web server.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure.
    async fn run(&self) -> Result<(), WebServerError>;
}

/// Represents the context for running a web server with a specific strategy.
pub struct WebServerContext<T: WebServerStrategy> {
    strategy: T,
}

impl<T: WebServerStrategy> WebServerContext<T> {
    /// Creates a new `WebServerContext` with the given strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The web server strategy to use.
    pub fn new(strategy: T) -> Self {
        WebServerContext { strategy }
    }

    /// Runs the web server using the configured strategy.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure.
    pub async fn run(&self) -> Result<(), WebServerError> {
        self.strategy.run().await
    }
}

/// Represents an Axum-based web server implementation.
pub struct AxumWebServer {
    addr: SocketAddr,
    router: Router,
    db_pool: DatabasePool,
}

impl AxumWebServer {
    /// Creates a new `AxumWebServer` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration.
    /// * `db_pool` - The database connection pool.
    ///
    /// # Returns
    ///
    /// Returns a new `AxumWebServer` instance.
    pub fn new(config: &Config, db_pool: DatabasePool) -> Self {
        let addr = SocketAddr::new(config.server.host.parse().unwrap(), config.server.port);
        let router = Router::new();
        AxumWebServer { addr, router, db_pool }
    }
}

#[async_trait]
impl WebServerStrategy for AxumWebServer {
    async fn run(&self) -> Result<(), WebServerError> {
        info!("Starting server on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).await.map_err(|e| {
            error!("Failed to bind to address: {}", e);
            WebServerError::ServerError(e.to_string())
        })?;

        axum::serve(listener, self.router.clone()).await.map_err(|e| {
            error!("Server error: {}", e);
            WebServerError::ServerError(e.to_string())
        })
    }
}
