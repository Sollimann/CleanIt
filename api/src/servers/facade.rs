// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_server::Roomba;
use protos::SensorData;
use protos::{Odometry, OdometryRequest};
use protos::{SensorsReceived, SensorsRequest};
extern crate proc_macro;
use crate::servers::endpoints::RoombaService;
use crate::servers::utils::SyncBoxStream;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl Roomba for RoombaService {
    async fn send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<SensorData>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        self.handle_send_sensor_stream(request).await
    }

    type GetSensorDataStream = SyncBoxStream<'static, Result<SensorData, Status>>;

    async fn get_sensor_data(
        &self,
        request: Request<SensorsRequest>,
    ) -> Result<Response<Self::GetSensorDataStream>, Status> {
        self.handle_get_sensor_data(request).await
    }

    type GetOdometryRawStream = SyncBoxStream<'static, Result<Odometry, Status>>;

    async fn get_odometry_raw(
        &self,
        request: Request<OdometryRequest>,
    ) -> Result<Response<Self::GetOdometryRawStream>, Status> {
        self.handle_get_odometry_raw(request).await
    }
}
