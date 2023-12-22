use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub async fn init_connection() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("localhost")
        .await
}
