use axum::Router;
use axum::routing::{get, post};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::init::{init, server_start_log};

mod config;
mod init;
mod redis;
mod db;
mod handler;
mod error;


#[tokio::main]
async fn main() {

    init();

    //启动axum web服务器
    let app = Router::new()
        .route("/login", post(handler::user::login))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
    ;


    let listener = tokio::net::TcpListener::bind("127.0.0.1:7002").await.unwrap();

    server_start_log();

    axum::serve(listener, app).await.unwrap();
}
