use std::time::Duration;
use lazy_static::lazy_static;
use log::info;
use redis::RedisConnectionInfo;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::{info_span, span};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
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

    // let file_appender = RollingFileAppender::new(Rotation::DAILY, "/Users/xzq/data/logs", "api.log");
    // let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);


    //1. 日志初始化
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // 定义 span 生命周期事件的记录
        // .with_writer(file_writer)
        .init();
}

pub(super) fn server_start_log() {
    let span = info_span!("server_start_log");
    let _enter = span.enter();

    println!("{}", common::banner::api_banner());
    info!("✅ Api Server start success, {}",API_CONFIG.server.address);
}

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_1() {
        let handler=thread::spawn(|| {
            let v1 = 2;
            let v2 = 3;
            println!("hello,thread")
        });

        handler.join().unwrap();
    }
}