use std::error::Error;

use chrono::{TimeZone, Utc};
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

impl<'s,T> ResponseResult<'s,T> where T: Send + Sync {

   pub fn ok(target:T)->ResponseResult<'s,T> {
      ResponseResult {
         code: "200",
         message: "ok",
         timestamp: Utc::now().timestamp(),
         data: target,
      }
   }

   pub fn fail(code:&'s str, msg:&'s str)->ResponseResult<'s,()> {
      ResponseResult {
         code:code,
         message: msg,
         timestamp: Utc::now().timestamp(),
         data: (),
      }
   }
}