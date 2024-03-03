use crate::{config::Config, usecases::{user}};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;


#[derive(Clone)]
pub struct ApiContext {
    cfg: Config,
    db: PgPool,
}

#[derive(Clone)]
pub struct ApiResponse<T: Serialize> {
    pub status: StatusCode,
    pub message: &'static str,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn response(&self) -> Response {
        (
            self.status,
            Json(json!({"message": self.message, "data": self.data})),
        )
            .into_response()
    }
}

pub async fn serve(cfg: Config, db: PgPool) -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/me", get(user::get_me))
        .route("/auth/login", post(user::post_auth))
        .route("/auth/signup", post(user::post_signup))
        .with_state(ApiContext { cfg, db })
}
