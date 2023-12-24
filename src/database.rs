use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_connection(env: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::new(5, 0))
        .connect(env)
        .await
        .unwrap_or_else(|_| panic!("Database Connection failure"))
}
