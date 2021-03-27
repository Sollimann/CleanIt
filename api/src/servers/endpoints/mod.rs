mod get_odometry_raw;
mod get_sensor_data;
mod send_sensor_stream;

// grpc theads
use crossbeam_channel::{bounded, Receiver, Sender};

// get odometry struct
// use async_std::sync::{Arc, Mutex};
use autonomy::slam::odometry::odometry::OdometryStamped;
use std::sync::Arc;
use tokio::sync::Mutex;

// get custom protos
use proto::roomba_service_protos as protos;
use protos::SensorData;

#[derive(Debug)]
pub struct RoombaService {
    pub rx: Receiver<SensorData>,
    tx: Sender<SensorData>,

    // https://kitsu.me/posts/2020_06_01_mutex_in_async_world
    pub odom_raw: Arc<Mutex<OdometryStamped>>,
}

impl RoombaService {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);
        let m = Mutex::new(OdometryStamped::init(0, 0));
        let odom_raw = Arc::new(m);
        Self { rx, tx, odom_raw }
    }
}
