use axum::{
    Router,
    routing::get
};
use axum::routing::post;
use sqlx::Pool;
use sqlx::postgres::{ PgPool };
use rustapi::logger;
use rustapi::routes;
use rustapi::database;

#[tokio::main]
async fn main() {
    logger::setup();
    tracing_subscriber::fmt::init();
    let _db = match database::init_connection().await.unwrap() {
        Pool(T) => T,
        Err(E) => panic!("{}", E),
    };


    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/auth/login", post(routes::auth::post_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
