use std::future::Future;
use anyhow::anyhow;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::Json;
use log::{error, info};
use serde::{Deserialize, Serialize};

use common::util::jwt;

use crate::handler::ResponseResult;
use crate::redis::redis_client::RedisOps;
use crate::redis::USER_TOKEN;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginReq {
    pub account_id: u32,
    pub credential: String,
}


pub async fn login(header_map: &HeaderMap,login_req: Json<LoginReq>) ->Json<ResponseResult<&'static str>> {

    //1. 查看缓存是否命中
    let redis_ops=RedisOps::connect().await.unwrap();

    match verify_user(&header_map,redis_ops).await {
        Ok(_) => {
            let data = header_map.get(AUTHORIZATION).unwrap().to_str().unwrap().clone();
            return Json(
                ResponseResult {
                    code: 200,
                    message: "ok",
                    timestamp:11111,
                    data: data
                }
            );
        }
        Err(err)=>{
            info!("direct login failed: {}.", err.to_string());
        }
    }

    info!("{:#?}",header_map);

    info!("{:#?}",login_req);

    Json(
        ResponseResult{
            code:200,
            message:"ok",
            timestamp: 123,
            data:"String"
        }
    )

}

pub async fn verify_user(headers: &HeaderMap, mut redis_ops: RedisOps) -> anyhow:: Result<u64> {
    let token = match headers.get(AUTHORIZATION) {
        None => { return Err(anyhow!("token is not empty!")); }
        Some(token) => {
            token.to_str().unwrap()
        }
    };

    let user_id = match jwt::audience_of_token(token) {
        Ok(user_id) => user_id,
        Err(e) => { return Err(anyhow!("token is invalid:{}",e)); }
    };

    let redis_key = format!("{}{}", USER_TOKEN, user_id);
    let token_key: String = match redis_ops.get::<String>(&redis_key) {
        Ok(token_key) => token_key,
        Err(_err) => { return Err(anyhow!("user not login")); }
    };


    let _res = match jwt::verify_token(token, token_key.as_bytes(), user_id).await {
        Ok(res) => res,
        Err(err) => return Err(anyhow!("token is invalid: {}",err))
    };

    Ok(user_id)
}
