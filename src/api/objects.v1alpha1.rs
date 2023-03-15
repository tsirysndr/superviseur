#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub command: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "9")]
    pub auto_restart: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub pid: u32,
    #[prost(uint32, tag = "4")]
    pub ppid: u32,
    #[prost(string, tag = "5")]
    pub command: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub up_time: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub working_directory: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub project: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub log_file: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub stderr_file: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub auto_restart: bool,
    #[prost(string, repeated, tag = "14")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "15")]
    pub service_id: ::prost::alloc::string::String,
}
