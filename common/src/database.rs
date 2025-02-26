use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::config::Settings;
use crate::errors::Result;

pub async fn connect_database(config: &Settings) -> Result<PgPool> {

    // Always use real database connection
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await
        .map_err(|e| crate::errors::AppError::Database(e.to_string()))?;
    
    Ok(pool)
}
