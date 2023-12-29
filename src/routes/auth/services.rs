use crate::routes::{ApiContext, ApiResponse};
use axum::{
    extract::{self, Json, State},
    http::StatusCode,
    response::Response,
};
use serde_json::{json, Value};

const DEFAULT_PASSWORD: &str = "defaultpassword";

pub async fn post_signup(extract::Json(payload): extract::Json<super::PostSignup>) -> Json<Value> {
    Json(json!({ "message": "success", "data": payload }))
}

pub async fn post_auth(
    State(state): State<ApiContext>,
    Json(payload): Json<super::AuthLogin>,
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
