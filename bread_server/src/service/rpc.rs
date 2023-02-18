// rpc.rs -  Here's our implementation of our various rpc functions
// inject our generated rpc services here
use std::sync::Arc;
pub mod bread_service {
    tonic::include_proto!("bread_service");
}

/// load key structures we injected into our sub-namespace into our main namespace
use bread_service::{
    bread_service_server::{BreadService, BreadServiceServer},
    GetUsersRequest, GetUsersResponse,
};

use diesel::SqliteConnection;
use tonic::{transport::Server, Request, Response, Status};

//BSImpl - a concrete implementation of the breadservice, requires a db connection to actually work!
pub struct BSImpl {
    conn: Arc<SqliteConnection>,
}
