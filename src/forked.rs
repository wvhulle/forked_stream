use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::Stream;

use crate::{bridge::ForkBridge, shared_bridge::CloneableForkBridge};

/// A wrapper around a stream that implements `Clone`.
pub struct ForkedStream<BaseStream>(pub CloneableForkBridge<BaseStream>)
where
    BaseStream: Stream<Item: Clone>;

impl<BaseStream> From<CloneableForkBridge<BaseStream>> for ForkedStream<BaseStream>
where
    BaseStream: Stream<Item: Clone>,
{
    fn from(bridge: CloneableForkBridge<BaseStream>) -> Self {
        bridge.new_fork()
    }
}

impl<BaseStream> Stream for ForkedStream<BaseStream>
where
    BaseStream: Stream<Item: Clone>,
{
    type Item = BaseStream::Item;

    fn poll_next(self: Pin<&mut Self>, new_context: &mut Context) -> Poll<Option<Self::Item>> {
        self.lock().unwrap().handle_fork(new_context.waker())
    }
}

impl<BaseStream> Clone for ForkedStream<BaseStream>
where
    BaseStream: Stream<Item: Clone>,
{
    fn clone(&self) -> Self {
        ForkedStream(self.0.clone())
    }
}

impl<BaseStream> Deref for ForkedStream<BaseStream>
where
    BaseStream: Stream<Item: Clone>,
{
    type Target = Arc<Mutex<ForkBridge<BaseStream>>>;

    fn deref(&self) -> &Self::Target {
        &self.0.0
    }
}

impl<BaseStream> DerefMut for ForkedStream<BaseStream>
where
    BaseStream: Stream<Item: Clone>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.0
    }
}
