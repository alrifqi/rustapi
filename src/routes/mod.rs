use crate::config::Config;
use axum::{
    routing::{get, post},
    Router,
};

pub mod auth;

pub async fn serve(_: Config) -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/me", get(auth::services::get_me))
        .route("/auth/login", post(auth::services::post_auth))
        .route("/auth/signup", post(auth::services::post_signup))
}
