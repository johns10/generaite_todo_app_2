use config::{Config as ConfigRs, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

/// Represents the application configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Application version, populated from Cargo.toml.
    pub version: String,
    /// Database configuration.
    pub database: DatabaseConfig,
    /// Server configuration.
    pub server: ServerConfig,
}

/// Database configuration.
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub max_connections: u32,
    pub connection_timeout: std::time::Duration,
}

/// Server configuration.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Loads the configuration from various sources.
    ///
    /// The configuration is loaded in the following order of priority:
    /// 1. Environment variables (highest priority)
    /// 2. Environment-specific config file (development.toml, test.toml, or production.toml)
    /// 3. Default config file (default.toml)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded `Config` if successful, or a `ConfigError` if an error occurred.
    pub fn load() -> Result<Self, ConfigError> {
        let run_mode = env::var("CARGO_RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = ConfigRs::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default.toml").required(false))
            // Add in the current environment file
            // Default to 'development' env if unspecified
            .add_source(File::with_name(&format!("config/{}.toml", run_mode)).required(false))
            // Add in settings from the environment (with a prefix of CARGO)
            // Eg.. `CARGO_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("CARGO"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        let mut config: Config = s.try_deserialize()?;

        // Populate version from Cargo.toml
        config.version = env!("CARGO_PKG_VERSION").to_string();

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        // Set some environment variables for testing
        env::set_var("CARGO_DATABASE_HOST", "test_host");
        env::set_var("CARGO_DATABASE_PORT", "5432");

        let config = Config::load().expect("Failed to load configuration");

        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_config() {
        let config = Config {
            version: "1.0.0".to_string(),
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                name: "test_db".to_string(),
                user: "postgres".to_string(),
                password: "password".to_string(),
                max_connections: 100,
                connection_timeout: std::time::Duration::from_secs(30),
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
        };

        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.database.name, "test_db");
        assert_eq!(config.database.user, "postgres");
        assert_eq!(config.database.password, "password");
        assert_eq!(config.database.max_connections, 100);
        assert_eq!(config.database.connection_timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
    }
}
