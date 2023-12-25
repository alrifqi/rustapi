use crate::config::Config;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::json;

pub mod auth;

#[derive(Clone)]
pub struct ApiContext {
    cfg: Config,
}

#[derive(Clone)]
pub struct ApiResponse<T: Serialize> {
    status: StatusCode,
    message: &'static str,
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    fn response(&self) -> Response {
        //(self.status, Json(json!(self.data))).into_response()
        (
            self.status,
            Json(json!({"message": self.message, "data": self.data})),
        )
            .into_response()
    }
}

pub async fn serve(_: Config) -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/me", get(auth::services::get_me))
        .route("/auth/login", post(auth::services::post_auth))
        .route("/auth/signup", post(auth::services::post_signup))
}
