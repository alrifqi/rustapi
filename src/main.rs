use axum::routing::post;
use axum::{routing::get, Router};
use clap::Parser;
use rustapi::config::Config;
use rustapi::database;
use rustapi::logger;
use rustapi::routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    logger::setup();
    tracing_subscriber::fmt::init();

    let config = Config::parse();
    let db = database::init_connection(&config.database_url).await;
    sqlx::migrate!().run(&db).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/auth/login", post(routes::auth::post_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
