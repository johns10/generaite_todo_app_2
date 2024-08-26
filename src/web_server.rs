use async_trait::async_trait;
use axum::Router;
use std::net::SocketAddr;
use thiserror::Error;

use crate::config::Config;

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
}

impl AxumWebServer {
    /// Creates a new `AxumWebServer` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The application configuration.
    ///
    /// # Returns
    ///
    /// Returns a new `AxumWebServer` instance.
    pub fn new(config: &Config) -> Self {
        // TODO: Implement this method
        unimplemented!("AxumWebServer::new() is not yet implemented")
    }
}

#[async_trait]
impl WebServerStrategy for AxumWebServer {
    async fn run(&self) -> Result<(), WebServerError> {
        // TODO: Implement the run method
        unimplemented!("AxumWebServer::run() is not yet implemented")
    }
}
