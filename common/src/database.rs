use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_database(config: &Config) -> Result<PgPool> {

    // Always use real database connection
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await?;
    
    Ok(pool)
}
