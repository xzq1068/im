use axum::Router;
use axum::routing::{get, post};

use crate::init::{init, server_start_log};

mod config;
mod init;
mod redis;
mod db;
mod handler;


#[tokio::main]
async fn main() {

    init();

    //启动axum web服务器
    let app = Router::new()
        .route("/login",get(handler::user::login));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7002").await.unwrap();

    server_start_log();

    axum::serve(listener, app).await.unwrap();
}
