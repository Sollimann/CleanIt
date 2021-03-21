// get custom protos
use proto::roomba_service_protos as protos;
use protos::{SensorData, SensorsReceived};

// grpc
use futures::StreamExt;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
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
