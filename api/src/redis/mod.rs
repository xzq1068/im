

//todo redis 未实现连接池 ，以后在实现 r2d2池化
pub mod redis_client;


pub(crate) static USER_TOKEN: &str = "USER_TOKEN_";
pub(crate) static JOIN_GROUP: &str = "JOIN_GROUP_";
pub(crate) static CHECK_CODE : &str = "CHECK_CODE_";
pub(crate) static LAST_ONLINE_TIME: &str = "LAST_ONLINE_TIME_";
pub(crate) static LAST_READ: &str = "LAST_READ_";
pub(crate) static USER_INBOX: &str = "USER_INBOX_";
pub(crate) static MSG_CACHE: &str = "MSG_CACHE_";
pub(crate) static ADD_FRIEND: &str = "ADD_FRIEND_";