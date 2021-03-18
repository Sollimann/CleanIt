// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_server::{Roomba, RoombaServer};
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// standard lib (threading, time, mutex, hashing)
use api::servers::facade::RoombaService;

// grpc tools
use drivers::roomba::startup::{shutdown, startup};
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:10000".parse().unwrap();
    println!("{:?}", addr);

    // creating a service
    let roomba_service = RoombaService::init();

    println!("Server listening on {}", addr);

    // adding our service to our server.
    let svc = RoombaServer::new(roomba_service);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
