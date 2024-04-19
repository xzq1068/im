use std::string::ToString;
use crate::handler::ResponseResult;

pub struct HandlerError {
    pub code: &'static str,
    pub message: &'static str,
}
impl HandlerError {
    pub const SERVER_ERROR: HandlerError = HandlerError { code: "api-1000", message: "服务器异常" };
    pub const USER_NOT_FOUND: HandlerError = HandlerError { code: "api-1001", message: "用户找不到" };
}