// get custom protos
use proto::roomba_service_protos as protos;
use protos::{SensorData, SensorsReceived};

// grpc
use futures::StreamExt;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
use colored::Colorize;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

impl RoombaService {
    pub async fn handle_send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<SensorData>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        let mut stream = request.into_inner();

        let mut received = SensorsReceived::default();

        while let Some(sensor_readings) = stream.next().await {
            println!("  ==> Sensors = {:?}", sensor_readings);
            let sensor_data: SensorData = match sensor_readings {
                Ok(data) => data,
                Err(error) => {
                    panic!(
                        "There was a problem reading sensor data stream: {:?}",
                        error
                    )
                }
            };

            if self.tx.send(sensor_data).await.is_err() {
                eprintln!("{}", "receiver dropped!".red());
            }

            // Increment the point count
            received.status = true;
            received.packet_count += 1;
        }

        Ok(Response::new(received))
    }
}
