use axum::{
    Router,
    routing::get
};
use axum::routing::post;

mod logger;
mod routes;

#[tokio::main]
async fn main() {
    logger::setup();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/auth/login", post(routes::auth::post_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
