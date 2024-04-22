use std::error::Error;
use axum::Json;
use chrono::{TimeZone, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use crate::error::HandlerError;

pub mod user;


//通用请求返回值
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ResponseResult<'s,T> where T: Send + Sync
{
   pub timestamp: i64,
   pub message: &'s str,
   pub code: &'s str,
   pub data: T,

}


// pub async fn global_error_handler(error: HandlerError) ->Json<ResponseResult<'static,()>>
// {
//     Json(ResponseResult {
//         code: error.code.parse().unwrap(),
//         message: error.message,
//         data: (),
//         timestamp: Utc::now().timestamp(),
//     })
// }
