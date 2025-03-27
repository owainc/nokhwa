use std::borrow::Cow;
use std::fmt::Debug;
use crate::error::NokhwaError;
use crate::frame_buffer::FrameBuffer;
use crate::frame_format::FrameFormat;
use crate::stream::{StreamHandle};
use crate::types::{CameraFormat, FrameRate, Resolution};

#[derive(Debug)]
pub struct Decoder<'stream, Video> where
    Video: Codec {
    video: Video,
    stream: &'stream mut StreamHandle
}

impl<'stream, Video> Decoder<'stream, Video> where Video: Codec {
    pub fn new(stream: &'stream mut StreamHandle, decoder: Video) -> Result<Self, NokhwaError> {
        let format = stream.format();
        
        let mut decoder = decoder;
        decoder.initialize(format)?;
        Ok(Self { video: decoder, stream })
    }
    
    pub fn 
}

#[cfg(feature = "async")]
#[derive(Debug)]
pub struct DecoderAsync<'stream, Video> where
    Video: CodecAsync {
    video: Video,
    stream_handle: &'stream mut StreamHandle
}

pub trait Codec: Debug {
    const ALLOWED_FORMATS: &'static [FrameFormat];
    
    fn initialize(&mut self, camera_format: CameraFormat) -> Result<(), NokhwaError>;
    
    fn stop(&mut self) -> Result<(), NokhwaError>;
    
    fn frame_format(&self) -> Result<FrameFormat, NokhwaError>;
    
    fn resolution(&self) -> Result<Resolution, NokhwaError>;
    
    fn frame_rate(&self) -> Result<FrameRate, NokhwaError>;
    
    fn set_frame_format(&mut self, frame_format: FrameFormat) -> Result<(), NokhwaError>;
    
    fn set_resolution(&mut self, resolution: Resolution) -> Result<(), NokhwaError>;
    
    fn set_frame_rate(&mut self, frame_rate: FrameRate) -> Result<(), NokhwaError>;
    
    fn decode_frame(&mut self, buffer: &FrameBuffer) -> Result<Cow<'_, [u8]>, NokhwaError>;
}

#[cfg(feature = "async")]
pub trait CodecAsync: Codec + Debug {}
