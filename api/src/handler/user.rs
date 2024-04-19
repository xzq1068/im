use axum::http::HeaderMap;
use axum::Json;
use log::info;
use serde::{Deserialize, Serialize};
use crate::handler::ResponseResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginReq {
    pub account_id: u32,
    pub credential: String,
}


pub async fn login(header_map: HeaderMap,login_req: Json<LoginReq>) ->Json<ResponseResult<'static,&'static str>> {

    //1. 查看缓存是否命中

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