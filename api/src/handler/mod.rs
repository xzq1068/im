use std::error::Error;

use chrono::TimeZone;
use serde::{Deserialize, Serialize};

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