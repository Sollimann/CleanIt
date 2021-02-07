use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

//We would use tokio::sync::mpsc for communicating between futures
use tokio::sync::mpsc;

// gRPC tools
use futures::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// our messages and services
use roombasensors::roomba_sensors_server::{RoombaSensors, RoombaSensorsServer};
use roombasensors::{LightBumper, SensorRequest, Sensors, SensorsReceived, Stasis};

// defining a struct for our service
#[derive(Debug)]
pub struct RoombaSensorsService;

pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl RoombaSensors for RoombaSensorsService {
    async fn send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<Sensors>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        let mut stream = request.into_inner();

        let mut received = SensorsReceived::default();

        while let Some(sensors) = stream.next().await {
            let sensors = sensors?;

            println!("  ==> Sensors = {:?}", sensors);

            // Increment the point count
            received.status = true;
            received.packet_count += 1;
        }

        Ok(Response::new(received))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:10000".parse().unwrap();

    // creating a service
    let sensors_service = RoombaSensorsService {};
    println!("Server listening on {}", addr);

    let svc = RoombaSensorsServer::new(sensors_service);

    // adding our service to our server.
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
