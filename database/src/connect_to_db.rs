use dotenvy::from_path;
use dotenvy::var;
use sqlx::Pool;
use sqlx::Postgres;
use std::path::Path;

pub async fn connect_db() -> Pool<Postgres> {
    // Explicitly load the .env file from the database directory
    let env_path = Path::new("../.env");
    from_path(env_path).expect("Failed to load .env file");

    // Read DATABASE_URL directly from the .env file
    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}