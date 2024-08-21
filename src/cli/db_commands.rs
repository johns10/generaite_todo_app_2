use crate::config::Config;
use postgres::{Client, NoTls};
use log::info;

/// Creates a new database based on the configuration.
pub fn create_database(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let connection_string = format!(
        "host={} port={} user={} password={} dbname=postgres",
        config.database.host,
        config.database.port,
        config.database.user,
        config.database.password
    );

    info!("Connecting to PostgreSQL server");
    let mut client = Client::connect(&connection_string, NoTls)?;

    let db_name = &config.database.name;
    info!("Creating database: {}", db_name);

    // Check if the database already exists
    let exists: bool = client
        .query_one(
            "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)",
            &[&db_name],
        )?
        .get(0);

    if exists {
        info!("Database '{}' already exists", db_name);
    } else {
        // Create the database
        let create_db_query = format!("CREATE DATABASE {}", db_name);
        client.execute(&create_db_query, &[])?;
        info!("Database '{}' created successfully", db_name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, DatabaseConfig};

    #[test]
    fn test_create_database() {
        // Create a test configuration
        let config = Config {
            version: "1.0.0".to_string(),
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                name: "test_db".to_string(),
                user: "postgres".to_string(),
                password: "password".to_string(),
            },
            // Add other required fields with dummy values
            server: Default::default(),
            jwt: Default::default(),
            logging: Default::default(),
            rate_limiting: Default::default(),
            feature_flags: Default::default(),
        };

        // Run the create_database function
        let result = create_database(&config);

        // Assert that the function executed without errors
        assert!(result.is_ok());
    }
}
