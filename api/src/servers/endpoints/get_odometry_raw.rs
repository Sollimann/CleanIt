// get custom protos
use proto::roomba_service_protos as protos;
use protos::{Odometry, OdometryRequest, SensorData};

// grpc
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
use crate::servers::utils::SyncBoxStream;
use async_std::sync::Arc;
use colored::Colorize;
use std::thread;

impl RoombaService {
    pub async fn handle_get_odometry_raw(
        &self,
        request: Request<OdometryRequest>,
    ) -> Result<Response<SyncBoxStream<'static, Result<Odometry, Status>>>, Status> {
        println!("request = {:?}", request);

        let (tx, rx) = mpsc::channel(1);
        let rx_clone = self.rx.clone();
        let odom_clone = self.odom_raw.clone();

        let mut count: u32 = 0;
        tokio::spawn(async move {
            while let Ok(sensor_data) = rx_clone.recv() {
                thread::sleep(Duration::from_millis(20));
                println!("{}", "sending data from server".green());
                let left = sensor_data.left_encoder_counts;
                let right = sensor_data.right_encoder_counts;
                let mut odom_raw = odom_clone.lock().await;
                let odom_msg: Odometry = odom_raw.compute_odom(left, right);
                tx.send(Ok(odom_msg)).await.unwrap();
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
