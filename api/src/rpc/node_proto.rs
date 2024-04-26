#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrNodeGroupIdUserListReq {
    #[prost(uint32, tag = "1")]
    pub node_id: u32,
    #[prost(uint64, tag = "2")]
    pub group_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrNodeGroupIdUserListResp {
    #[prost(uint64, repeated, tag = "1")]
    pub user_list: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhichNodeReq {
    #[prost(uint64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhichNodeResp {
    #[prost(uint32, tag = "1")]
    pub node_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMsgReq {
    #[prost(uint64, tag = "1")]
    pub sender: u64,
    #[prost(uint64, tag = "2")]
    pub receiver: u64,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    #[prost(uint32, tag = "4")]
    pub version: u32,
    #[prost(uint32, tag = "5")]
    pub r#type: u32,
    #[prost(string, tag = "6")]
    pub payload: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub extension: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMsgResp {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub err_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecorderListReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecorderListResp {
    #[prost(string, repeated, tag = "1")]
    pub address_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "2")]
    pub node_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhichToConnectReq {
    #[prost(uint64, tag = "1")]
    pub user_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WhichToConnectResp {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUserListReq {
    #[prost(uint64, tag = "1")]
    pub group_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUserListResp {
    #[prost(uint64, repeated, tag = "1")]
    pub user_list: ::prost::alloc::vec::Vec<u64>,
}


pub mod scheduler_client{
    use serde::de::StdError;

    pub struct SchedulerClient<T>{
        inner: tonic::client::Grpc<T>,
    }
    impl SchedulerClient<tonic::transport::channel>{
        pub async fn connect<D>(dst:D) -> Result<Self, tonic::transport::Error>
            where   D: std::convert::TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<dyn StdError>{
            let conn=tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new)

        }
    }

}