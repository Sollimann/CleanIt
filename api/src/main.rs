use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use drivers::roomba::drive;
use roombasensors::roomba_sensors_server::{RoombaSensors, RoombaSensorsServer};
use roombasensors::{LightBumper, Sensors, SensorsReceived, Stasis};

#[derive(Debug)]
struct RoombaSensorsService;

pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}

#[tonic::async_trait]
impl RoombaSensors for RoombaSensorsService {
    async fn send_sensor_stream(
        &self,
        _request: Request<tonic::Streaming<Sensors>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        unimplemented!()
    }
}

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    drive::drive_and_sense();
    //mode_commands();
}
