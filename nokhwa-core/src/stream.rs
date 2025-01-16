use crate::error::{NokhwaError, NokhwaResult};
use crate::frame_buffer::FrameBuffer;
use std::sync::Arc;
use derive_builder::Builder;
use flume::{Receiver, TryRecvError};

#[derive(Clone, Debug, Default, PartialEq, Builder)]
pub struct StreamConfiguration {
    buffer_size: Option<u32>,

}

pub trait StreamInnerTrait {
    fn configuration(&self) -> &StreamConfiguration;
    fn receiver(&self) -> Arc<Receiver<FrameBuffer>>;
    fn stop(&mut self) -> NokhwaResult<()>;
}

pub struct Stream {
    inner: Box<dyn StreamInnerTrait>,
}

impl Stream {
    pub fn new(inner: Box<dyn StreamInnerTrait>) -> Self {
        Self {
            inner,
        }
    }

    pub fn check_disconnected(&self) -> NokhwaResult<()> {
        if self.inner.receiver().is_disconnected() {
            return Err(NokhwaError::ReadFrameError(
                "stream is disconnected!".to_string(),
            ))
        }
        Ok(())
    }

    pub fn poll_frame(&self) -> NokhwaResult<FrameBuffer> {
        self.check_disconnected()?;

        self.inner
            .receiver()
            .recv()
            .map_err(|why| NokhwaError::ReadFrameError(why.to_string()))
    }

    pub fn try_poll_frame(&self) -> NokhwaResult<Option<FrameBuffer>> {
        self.check_disconnected()?;

        if self.inner.receiver().is_empty() {
            return Ok(None);
        }

        let possible_frame = self.inner
            .receiver()
            .try_recv();

        match possible_frame {
            Ok(f) => Ok(Some(f)),
            Err(why) => {
                match why {
                    TryRecvError::Empty => Ok(None),
                    TryRecvError::Disconnected => Err(NokhwaError::ReadFrameError(
                        "stream is disconnected!".to_string(),
                    ))
                }
            }
        }

    }

    #[cfg(feature = "async")]
    pub async fn await_frame(&self) -> NokhwaResult<FrameBuffer> {
        use futures::TryFutureExt;

        self.check_disconnected()?;

        self.inner
            .receiver()
            .recv_async()
            .map_err(|why| NokhwaError::ReadFrameError(why.to_string())).await
    }

    pub fn stop_stream(mut self) -> NokhwaResult<()> {
        self.inner.stop()?;
        Ok(())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        let _ = self.inner.stop();
    }
}
