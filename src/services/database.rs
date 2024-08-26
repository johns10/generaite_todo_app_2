use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use crate::config::DatabaseConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to the database: {0}")]
    ConnectionError(#[from] DbErr),
}

/// Represents a pool of database connections.
pub struct DatabasePool {
    connection: Arc<DatabaseConnection>,
}

impl DatabasePool {
    /// Creates a new DatabasePool instance.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the DatabaseConfig containing connection details.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the DatabasePool if successful, or a DatabaseError if an error occurred.
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.name
        );

        let connection = Database::connect(&connection_string)
            .await
            .map_err(DatabaseError::ConnectionError)?;

        Ok(Self {
            connection: Arc::new(connection),
        })
    }

    /// Returns a reference to the database connection.
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_database_pool_creation() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            name: "test_db".to_string(),
            user: "postgres".to_string(),
            password: "password".to_string(),
            max_connections: 5,
            connection_timeout: Duration::from_secs(5),
        };

        let result = DatabasePool::new(&config).await;
        assert!(result.is_ok(), "Failed to create database pool");
    }
}
