use std::collections::HashMap;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

//We would use tokio::sync::mpsc for communicating between futures
use tokio::sync::mpsc;

// gRPC tools
use futures::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:10000".parse().unwrap();

    // // creating a service
    // let sensors_service = RoombaSensorsService {};
    //
    // println!("Server listening on {}", addr);
    //
    // let svc = RoombaSensorsServer::new(sensors_service);
    //
    // // adding our service to our server.
    // Server::builder().add_service(svc).serve(addr).await?;
    //
    // Ok(())
}
