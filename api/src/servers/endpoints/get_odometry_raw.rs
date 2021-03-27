// get custom protos
use proto::roomba_service_protos as protos;
use protos::{Odometry, OdometryRequest};

// grpc
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
use crate::servers::utils::SyncBoxStream;
use colored::Colorize;
use std::thread;

impl RoombaService {
    pub async fn handle_get_odometry_raw(
        &self,
        request: Request<OdometryRequest>,
    ) -> Result<Response<SyncBoxStream<'static, Result<Odometry, Status>>>, Status> {
        unimplemented!("todo")
    }
}
