use async_once_cell::OnceCell;
use sea_orm::{Database, DatabaseConnection};
use std::env;

static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

/// Initialize the global database connection
pub async fn initialize_db_connection() -> Result<(), sea_orm::DbErr> {
    // Fetch the database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    // Connect to the database
    let connection = Database::connect(&database_url).await?;

    // Store the connection in the OnceCell
    DB_CONNECTION.get_or_init(async { connection }).await;

    Ok(())
}

/// Access the global database connection
pub async fn get_db_connection() -> &'static DatabaseConnection {
    DB_CONNECTION
        .get()
        .expect("Database connection has not been initialized")
}
