use std::any::Any;
use sqlx::{Error, Pool, Postgres};
use sqlx::database::Database;
use sqlx::postgres::{PgPool, PgPoolOptions };

pub async fn init_connection() -> Result<Pool<Postgres>, Error>{
    PgPoolOptions::new()
        .max_connections(5)
        .connect("localhost")
        .await
}