use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_connection(env: &str) -> Pool<Postgres> {
    let pool_conn = PgPoolOptions::new().max_connections(5).connect(env).await;
    match pool_conn {
        Ok(p) => p,
        Err(err) => panic!("Connection failure: {}", err),
    }
}
