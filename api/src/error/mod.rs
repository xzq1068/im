use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;

use crate::handler::ResponseResult;

pub enum HandlerError{
    ServerError,
    UserNotFound,
}

impl HandlerError{
    pub fn convert_obj(&mut self)->HandlerErrorObj{
        match self {
            HandlerError::ServerError => HandlerErrorObj { code: "api-1000", message:"服务器异常" },
            HandlerError::UserNotFound => HandlerErrorObj { code: "api-1001", message:"用户找不到" },
        }
    }
}

pub struct HandlerErrorObj {
    pub code: &'static str,
    pub message: &'static str,
}

impl IntoResponse for  HandlerError{
    fn into_response(mut self) -> Response {
        let err_obj = self.convert_obj();
        let response = ResponseResult {
            code: err_obj.code,
            message: err_obj.message,
            timestamp: Utc::now().timestamp(),
            data: (),
        };

        // 将错误响应序列化为JSON，并创建HTTP响应
        let json_body=serde_json::to_string(&response).unwrap();

        axum::http::Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_body))
            .unwrap()
    }
}
