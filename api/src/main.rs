use axum::{Router, ServiceExt};
use axum::routing::{get, post};
use log::info;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::init::{API_CONFIG, init, server_start_log};

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
        .route("/test", get(handler::user::test))
        .layer(
            ServiceBuilder::new()
                // .layer(HandleErrorLayer::new(handler::handler_error))
                .layer(TraceLayer::new_for_http())
        );

    let listener = tokio::net::TcpListener::bind(&API_CONFIG.server.address).await.unwrap();

    server_start_log();



    axum::serve(listener, app).await.unwrap();
}
