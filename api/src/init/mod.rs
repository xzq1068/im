use std::time::Duration;
use lazy_static::lazy_static;
use log::info;
use redis::RedisConnectionInfo;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::config::{ApiConfig, load_config};


lazy_static! {

    pub static ref API_CONFIG:ApiConfig={

        load_config()

    };

    pub static ref DB_POOL:Pool<Postgres>={

        let url=format!("postgres://{}:{}@{}/{}",API_CONFIG.datasource.username,API_CONFIG.datasource.password,API_CONFIG.datasource.address,API_CONFIG.datasource.db_name);

        PgPoolOptions::new()
            .max_connections(API_CONFIG.datasource.pool_max_connection)
            .connect_lazy(&url)
            .expect("❌ init datasource pool error")

    };

}

pub(super) fn init() {
    //1. 日志初始化
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

pub(super) fn server_start_log() {
    println!("{}", common::banner::api_banner());
    info!("✅ Api Server start success, {}/{}","127.0.0.1",API_CONFIG.server.port);
}