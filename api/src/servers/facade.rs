// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_server::{Roomba, RoombaServer};
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// get standard library utils
use async_std::task;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
extern crate proc_macro;
use proc_macro::TokenStream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

// gRPC tools
use crate::servers::endpoints::RoombaService;
// use futures::{Stream, StreamExt};
// use proc_macro::TokenStream;
use crate::servers::utils::SyncBoxStream;
use std::thread;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl Roomba for RoombaService {
    async fn send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<SensorData>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        self.handle_send_sensor_stream(request).await
    }

    // define type alias
    // #[rustfmt::skip]
    // type GetSensorDataStream = Pin<Box<dyn Stream<Item = Result<SensorData, Status>>
    // + Send
    // + Sync
    // + 'static
    // >>;
    type GetSensorDataStream = SyncBoxStream<'static, Result<SensorData, Status>>;

    async fn get_sensor_data(
        &self,
        request: Request<SensorsRequest>,
    ) -> Result<Response<Self::GetSensorDataStream>, Status> {
        println!("request = {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        // tokio::spawn(async {
        //     loop {
        //         if let Some(data) = self.pop_sensor_data_from_buffer() {
        //             tx.send(Ok(data)).await.unwrap();
        //         }
        //     }
        // });
        if let Some(data) = self.pop_sensor_data_from_buffer() {
            tx.send(Ok(data)).await.unwrap();
        }

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}
