use std::error::Error;
use std::future::Future;

use anyhow::anyhow;
use axum::error_handling::HandleError;
use axum::http::header::AUTHORIZATION;
use axum::http::HeaderMap;
use axum::Json;
use axum::response::Response;
use hmac::{Hmac, Mac};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use common::util::jwt;
use common::util::jwt::simple_token;

use crate::db::user::User;
use crate::error::HandlerError;
use crate::handler::ResponseResult;
use crate::redis::redis_client::RedisOps;
use crate::redis::USER_TOKEN;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginReq {
    pub account_id: i64,
    pub credential: String,
}

type HmacSha256 = Hmac<Sha256>;

pub async fn test() -> Result<Json<ResponseResult<'static, String>>, HandlerError> {
    Err(HandlerError::UserNotFound)
}

pub async fn login(header_map: HeaderMap, login_req: Json<LoginReq>) -> Result<Json<ResponseResult<'static, String>>, HandlerError> {
    //1. 查看缓存是否命中
    let mut  redis_ops = RedisOps::connect().await.unwrap();

    match verify_user(&header_map, redis_ops).await {
        Ok(_) => {
            return Ok(
                Json(
                    ResponseResult {
                        code: "200",
                        message: "ok",
                        timestamp: 11111,
                        data: header_map.get(AUTHORIZATION).to_owned().unwrap().to_str().unwrap().to_string(),
                    }
                )
            );
        }
        Err(err) => {
            error!("direct login failed: {}.", err.to_string());
        }
    }

    let user = match User::get_by_account_id(login_req.account_id).await {
        Ok(user) => user,
        Err(err) => {
            return Err(HandlerError::UserNotFound);
        }
    };
    //使用HMAC-SHA256堆成加密算法加盐
    let mut hmac = HmacSha256::new_from_slice(user.salt.as_bytes()).unwrap();
    hmac.update(login_req.credential.as_bytes());
    let result = hmac.finalize().into_bytes();
    let result_str = hex::encode(result);

    //比较用户凭证
    if result_str != user.credential {
      return   Err(HandlerError::UserAuthFail)
    };

    let val=salt(12);
    let mut key=format!("{}{}", USER_TOKEN, login_req.account_id);

    let mut  redis_ops_2 = RedisOps::connect().await.unwrap();

    match redis_ops_2.set(&key, &val).await {
        Ok(_) => {}
        Err(error) => { return Err(HandlerError::ServerError)}
    }

    let token=simple_token(val.as_bytes(), login_req.account_id);


    Ok(Json(ResponseResult::ok(token)))
}

pub async fn verify_user(headers: &HeaderMap, mut redis_ops: RedisOps) -> anyhow::Result<i64> {
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
    let token_key: String = match redis_ops.get::<String>(&redis_key).await {
        Ok(token_key) => token_key,
        Err(_err) => { return Err(anyhow!("user not login")); }
    };


    let _res = match jwt::verify_token(token, token_key.as_bytes(), user_id).await {
        Ok(res) => res,
        Err(err) => return Err(anyhow!("token is invalid: {}",err))
    };

    Ok(user_id)
}
#[allow(unused)]
#[inline]
pub fn salt(length: usize) -> String {

    let length = if length > 32 { 32 } else { length };
    let v4 = Uuid::new_v4();
    let string = uuid::Uuid::new_v4().to_string().replace("-", "M");
    String::from_utf8_lossy(&string.as_bytes()[0..length])
        .to_string()
        .to_uppercase()

}

#[cfg(test)]
mod tests{
    use hmac::Mac;
    use crate::handler::user::HmacSha256;

    #[test]
    pub fn test_salt() {

        let mut hmac = HmacSha256::new_from_slice("xzq".as_bytes()).unwrap();
        hmac.update("isMy".as_bytes());
        let result = hmac.finalize().into_bytes();
        let result_str = hex::encode(result);

        println!("{result_str}")
    }
}