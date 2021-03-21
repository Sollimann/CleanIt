use tokio::sync::mpsc;
use tonic::Status;

use futures_core::Stream;
use std::pin::Pin;

pub type SyncBoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a + Send + Sync>>;
