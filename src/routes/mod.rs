use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{PgPool, Postgres};

use crate::config::Config;

pub mod auth;

pub async fn serve(config: Config) -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/auth/login", post(auth::post_auth))
}
