// rpc.rs -  Here's our implementation of our various rpc functions
// inject our generated rpc services here
pub mod bread_service {
    tonic::include_proto!("bread_service");
}

/// load key structures we injected into our sub-namespace into our main namespace
use bread_service::{
    bread_service_server::{BreadService, BreadServiceServer},
    GetUsersRequest, GetUsersResponse,
};
