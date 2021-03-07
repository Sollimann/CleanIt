// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_server::{Roomba, RoombaServer};
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// get standard library utils
use std::collections::HashMap;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

// gRPC tools
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr: String = "[::1]:10000".parse().unwrap();
    println!("{:?}", addr);
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
    Ok(())
}
