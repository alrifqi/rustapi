use std::sync::Arc;

use crate::{
    repository::user,
    routes::{ApiContext, ApiResponse},
    AuthLogin, PostSignup,
};
use axum::{
    extract::{self, Json},
    http::StatusCode,
    response::Response,
    Extension,
};
use serde_json::{json, Value};

const DEFAULT_PASSWORD: &str = "defaultpassword";

pub async fn post_signup(
    Extension(ctx): Extension<ApiContext>,
    extract::Json(payload): extract::Json<PostSignup>,
) -> Json<Value> {
    user::get_user_by_email(ctx.db_conn(), payload.email.clone());
    Json(json!({ "message": "success", "data": payload }))
}

pub async fn post_auth(
    Extension(ctx): Extension<ApiContext>,
    Json(payload): Json<AuthLogin>,
) -> Response {
    let validation = payload.validate();
    if !validation.message.is_empty() {
        return ApiResponse {
            status: StatusCode::BAD_REQUEST,
            message: "error",
            data: json!({}),
        }
        .response();
    }
    ApiResponse {
        status: StatusCode::CREATED,
        message: "success",
        data: json!(validation.payload),
    }
    .response()
}

pub async fn get_me() -> Json<Value> {
    Json(json!({ "message": "success"}))
}
