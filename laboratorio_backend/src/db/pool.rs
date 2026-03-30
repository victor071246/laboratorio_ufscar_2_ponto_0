use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn criar_pool(database_url: &str) -> Result
<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_seconds(5)
        .connect(database_url))
        .await
}