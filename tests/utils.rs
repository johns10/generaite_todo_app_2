//! Test utilities for the To-Do application.
//!
//! This module provides utility functions and structures to facilitate testing
//! of the Rust-based To-Do application. It includes database setup and teardown,
//! test app configuration, mock data generation, authentication helpers, custom
//! assertions, and fixtures.

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    Router,
};
use sea_orm::{
    Database, DatabaseConnection, EntityTrait, MockDatabase, MockExecResult, Transaction,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{category, task, user};

/// Sets up a test database using Sea-ORM with a connection pool.
///
/// # Returns
///
/// A `Result` containing the `DatabaseConnection` if successful, or an error if setup fails.
pub async fn setup_test_db() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let db_url = "sqlite::memory:".to_string();
    let db = Database::connect(db_url).await?;
    
    // Run migrations or create tables as needed
    // This is a placeholder and should be replaced with actual migration logic
    // migrate::run(&db).await?;

    Ok(db)
}

/// Tears down the test database after tests.
///
/// # Arguments
///
/// * `db` - The `DatabaseConnection` to be closed.
pub async fn teardown_test_db(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    db.close().await?;
    Ok(())
}

/// Returns a configured Axum Router instance for testing.
///
/// # Arguments
///
/// * `db` - The `DatabaseConnection` to be used with the router.
///
/// # Returns
///
/// An `axum::Router` instance configured for testing.
pub fn get_test_app(db: DatabaseConnection) -> Router {
    let config = Config::load().expect("Failed to load config");
    let shared_state = Arc::new(Mutex::new(db));
    
    // This is a placeholder and should be replaced with actual router setup
    Router::new()
        .with_state(shared_state)
        // Add your routes and middleware here
}

/// Generates a mock Task instance.
///
/// # Returns
///
/// A `task::Model` instance with mock data.
pub fn mock_task() -> task::Model {
    task::Model {
        id: Uuid::new_v4(),
        title: "Mock Task".to_string(),
        description: Some("This is a mock task for testing".to_string()),
        status: "pending".to_string(),
        due_date: Some(chrono::Utc::now().naive_utc()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        user_id: Uuid::new_v4(),
        category_id: Some(Uuid::new_v4()),
    }
}

/// Generates a mock User instance.
///
/// # Returns
///
/// A `user::Model` instance with mock data.
pub fn mock_user() -> user::Model {
    user::Model {
        id: Uuid::new_v4(),
        username: "mockuser".to_string(),
        email: "mock@example.com".to_string(),
        password_hash: "mockhash".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    }
}

/// Generates a mock Category instance.
///
/// # Returns
///
/// A `category::Model` instance with mock data.
pub fn mock_category() -> category::Model {
    category::Model {
        id: Uuid::new_v4(),
        name: "Mock Category".to_string(),
        description: Some("This is a mock category for testing".to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        user_id: Uuid::new_v4(),
    }
}

/// Generates an authenticated request for testing protected routes.
///
/// # Arguments
///
/// * `method` - The HTTP method as a `&str`.
/// * `uri` - The request URI as a `&str`.
/// * `body` - The request body as a `serde_json::Value`.
/// * `token` - The authentication token as a `&str`.
///
/// # Returns
///
/// A `Request<Body>` with authentication headers set.
pub fn authenticated_request(
    method: &str,
    uri: &str,
    body: serde_json::Value,
    token: &str,
) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(body.to_string()))
        .unwrap()
}

/// Asserts that a response has a specific status code and body.
///
/// # Arguments
///
/// * `response` - The `axum::response::Response` to check.
/// * `expected_status` - The expected `StatusCode`.
/// * `expected_body` - The expected body as a `serde_json::Value`.
///
/// # Returns
///
/// A `Result` indicating whether the assertion passed or failed.
pub async fn assert_response(
    response: axum::response::Response,
    expected_status: StatusCode,
    expected_body: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(response.status(), expected_status);
    
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body: serde_json::Value = serde_json::from_slice(&body)?;
    
    assert_eq!(body, expected_body);
    
    Ok(())
}

/// A fixture for creating multiple related entities for testing.
pub struct TestFixture {
    pub user: user::Model,
    pub category: category::Model,
    pub tasks: Vec<task::Model>,
}

impl TestFixture {
    /// Creates a new TestFixture with a user, category, and specified number of tasks.
    ///
    /// # Arguments
    ///
    /// * `task_count` - The number of mock tasks to create.
    ///
    /// # Returns
    ///
    /// A new `TestFixture` instance.
    pub fn new(task_count: usize) -> Self {
        let user = mock_user();
        let category = mock_category();
        let tasks = (0..task_count).map(|_| mock_task()).collect();

        TestFixture {
            user,
            category,
            tasks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_and_teardown_db() {
        let db = setup_test_db().await.expect("Failed to set up test DB");
        teardown_test_db(db).await.expect("Failed to tear down test DB");
    }

    #[test]
    fn test_mock_data_generation() {
        let task = mock_task();
        let user = mock_user();
        let category = mock_category();

        assert!(!task.title.is_empty());
        assert!(!user.username.is_empty());
        assert!(!category.name.is_empty());
    }

    #[test]
    fn test_authenticated_request() {
        let req = authenticated_request(
            "POST",
            "/api/tasks",
            json!({"title": "Test Task"}),
            "test_token",
        );

        assert_eq!(req.method(), "POST");
        assert_eq!(req.uri(), "/api/tasks");
        assert_eq!(
            req.headers().get("Authorization").unwrap(),
            "Bearer test_token"
        );
    }

    #[test]
    fn test_fixture_creation() {
        let fixture = TestFixture::new(3);
        assert_eq!(fixture.tasks.len(), 3);
        assert!(!fixture.user.username.is_empty());
        assert!(!fixture.category.name.is_empty());
    }
}
