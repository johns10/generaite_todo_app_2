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
    /// JWT configuration.
    pub jwt: JwtConfig,
    /// Logging configuration.
    pub logging: LoggingConfig,
    /// Rate limiting configuration.
    pub rate_limiting: RateLimitingConfig,
    /// Feature flags.
    pub feature_flags: FeatureFlags,
}

/// Database configuration.
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

/// Server configuration.
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// JWT configuration.
#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
}

/// Logging configuration.
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

/// Rate limiting configuration.
#[derive(Debug, Deserialize)]
pub struct RateLimitingConfig {
    pub requests: u32,
    pub duration: u64,
}

/// Feature flags.
#[derive(Debug, Deserialize)]
pub struct FeatureFlags {
    pub enable_registration: bool,
    pub enable_social_login: bool,
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
        env::set_var("CARGO_SERVER_PORT", "8080");

        let config = Config::load().expect("Failed to load configuration");

        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.version, env!("CARGO_PKG_VERSION"));
    }
}
