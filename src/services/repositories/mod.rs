use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// The `Repository` trait defines a set of CRUD operations that can be performed on a generic model type.
///
/// This trait is designed to be implemented by concrete repository types for specific models.
/// It provides a consistent interface for database operations across different model types.
///
/// # Type Parameters
///
/// * `M`: The model type this repository operates on.
/// * `E`: The error type returned by repository operations.
///
/// # Examples
///
/// ```
/// # use async_trait::async_trait;
/// # use your_crate::Repository;
/// struct User;
/// struct UserRepository;
///
/// #[async_trait]
/// impl Repository<User> for UserRepository {
///     type Error = std::io::Error;
///
///     async fn find_by_id(&self, id: i32) -> Result<Option<User>, Self::Error> {
///         // Implementation here
///         todo!()
///     }
///
///     // Other method implementations...
/// }
/// ```
#[async_trait]
pub trait Repository<M> {
    /// The error type returned by repository operations.
    type Error;

    /// Finds a model by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the model to find.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `Option<M>` if successful, or an `Error` if the operation fails.
    async fn find_by_id(&self, id: i32) -> Result<Option<M>, Self::Error>;

    /// Retrieves all models.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Vec<M>` if successful, or an `Error` if the operation fails.
    async fn find_all(&self) -> Result<Vec<M>, Self::Error>;

    /// Creates a new model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to create.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created model if successful, or an `Error` if the operation fails.
    async fn create(&self, model: M) -> Result<M, Self::Error>;

    /// Updates an existing model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to update.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the updated model if successful, or an `Error` if the operation fails.
    async fn update(&self, model: M) -> Result<M, Self::Error>;

    /// Deletes a model by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the model to delete.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure of the operation.
    async fn delete(&self, id: i32) -> Result<(), Self::Error>;
}

/// The `RepositoryContext` struct provides a shared database connection for repositories.
///
/// This struct is designed to be used as a dependency for concrete repository implementations,
/// allowing them to share a database connection across multiple repository instances.
///
/// # Examples
///
/// ```
/// # use sea_orm::DatabaseConnection;
/// # use std::sync::Arc;
/// # use your_crate::RepositoryContext;
/// let db_connection = Arc::new(DatabaseConnection::connect("database_url").await?);
/// let context = RepositoryContext::new(db_connection);
/// ```
pub struct RepositoryContext {
    /// The shared database connection.
    pub connection: Arc<DatabaseConnection>,
}

impl RepositoryContext {
    /// Creates a new `RepositoryContext` with the given database connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - An `Arc<DatabaseConnection>` representing the shared database connection.
    ///
    /// # Returns
    ///
    /// Returns a new `RepositoryContext` instance.
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}
