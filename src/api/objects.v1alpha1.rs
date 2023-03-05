#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub command: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "7")]
    pub auto_restart: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub pid: u32,
    #[prost(uint32, tag = "3")]
    pub ppid: u32,
    #[prost(string, tag = "4")]
    pub command: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub up_time: ::prost::alloc::string::String,
}
