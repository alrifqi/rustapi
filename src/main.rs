use axum::routing::post;
use axum::{routing::get, Router};
use rustapi::database;
use rustapi::logger;
use rustapi::routes;

#[tokio::main]
async fn main() {
    logger::setup();
    tracing_subscriber::fmt::init();
    let _db = match database::init_connection().await {
        Ok(pg) => pg,
        Err(err) => panic!("{}", err),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/auth/login", post(routes::auth::post_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
