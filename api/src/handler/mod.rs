use serde::{Deserialize, Serialize};

pub mod user;


//通用请求返回值
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ResponseResult<'a ,T> where T: Send+Sync+ 'static
{
    timestamp: i64,
    message: &'a str,
    code: u32,
    data: T,
}


