// get custom protos
use proto::roomba_service_protos as protos;
use protos::{SensorData, SensorsRequest};

// grpc
use tokio::sync::mpsc;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
use crate::servers::utils::SyncBoxStream;
use colored::Colorize;
use std::thread;

impl RoombaService {
    pub async fn handle_get_sensor_data(
        &self,
        request: Request<SensorsRequest>,
    ) -> Result<Response<SyncBoxStream<'static, Result<SensorData, Status>>>, Status> {
        println!("request = {:?}", request);

        let (tx, rx) = mpsc::channel(1);
        let rx_clone = self.rx.clone();

        let mut count: u32 = 0;
        tokio::spawn(async move {
            while let Ok(data) = rx_clone.recv() {
                thread::sleep(Duration::from_millis(20));
                println!("{}", "sending data from server".green());
                tx.send(Ok(data)).await.unwrap();
                count += 1;
                println!("count: {}", &count);
            }
            println!("{}", "failed sending data from server".red());
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}
