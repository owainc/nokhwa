use crate::{error::NokhwaError, frame_buffer::FrameBuffer, frame_format::FrameFormat};
use image::{ImageBuffer, Pixel};
use std::ops::{ControlFlow, Deref};

/// Trait to define a struct that can decode a [`FrameBuffer`]
pub trait Decoder<OutputPixel: Pixel> {
    /// Formats that the decoder can decode.
    const ALLOWED_FORMATS: &'static [FrameFormat];

    /// Container type for the decoder. Will be used for ImageBuffer
    type PixelContainer: Deref<Target = [OutputPixel::Subpixel]>;

    fn check_format(buffer: &FrameBuffer) -> ControlFlow<NokhwaError> {
        if !Self::ALLOWED_FORMATS.contains(&buffer.source_frame_format()) {
            return ControlFlow::Break(NokhwaError::ConversionError("unsupported".to_string()));
        }

        ControlFlow::Continue(())
    }

    /// Decode function.
    fn decode(
        &mut self,
        buffer: &FrameBuffer,
    ) -> Result<ImageBuffer<OutputPixel, Self::PixelContainer>, NokhwaError>;

    /// Decode to user-provided Buffer
    ///
    /// Incase that the buffer is not large enough this should error.
    fn decode_buffer(
        &mut self,
        buffer: &FrameBuffer,
        output: &mut [OutputPixel::Subpixel],
    ) -> Result<(), NokhwaError>;

    /// Decoder Predicted Size
    fn predicted_size_of_frame(buffer: &FrameBuffer) -> Option<usize> {
        if !Self::ALLOWED_FORMATS.contains(&buffer.source_frame_format()) {
            return None;
        }
        let res = buffer.resolution();
        Some(
            res.x() as usize
                * res.y() as usize
                * size_of::<OutputPixel::Subpixel>()
                * OutputPixel::CHANNEL_COUNT as usize,
        )
    }
}

/// Decoder that can be used statically (struct contains no state)
///
/// This is useful for times that a simple function is all that is required.
pub trait StaticDecoder<OutputPixel: Pixel>: Decoder<OutputPixel> {
    fn decode_static(
        buffer: &FrameBuffer,
    ) -> Result<ImageBuffer<OutputPixel, Self::PixelContainer>, NokhwaError>;

    fn decode_static_to_buffer(
        buffer: &FrameBuffer,
        output: &mut [OutputPixel::Subpixel],
    ) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait AsyncDecoder<OutputPixel: Pixel>: Decoder<OutputPixel> {
    /// Asynchronous decoder
    async fn decode_async(
        &mut self,
        buffer: &FrameBuffer,
    ) -> Result<ImageBuffer<OutputPixel, Self::PixelContainer>, NokhwaError>;

    /// Asynchronous decoder to user buffer.
    async fn decode_buffer(
        &mut self,
        buffer: &FrameBuffer,
        output: &mut [OutputPixel::Subpixel],
    ) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait AsyncStaticDecoder<OutputPixel: Pixel>:
    Decoder<OutputPixel> + AsyncDecoder<OutputPixel>
{
    /// Asynchronous decoder
    async fn decode_static_async(
        buffer: &FrameBuffer,
    ) -> Result<ImageBuffer<OutputPixel, Self::PixelContainer>, NokhwaError>;

    /// Asynchronous decoder to user buffer.
    async fn decode_static_buffer_async(
        buffer: &FrameBuffer,
        output: &mut [OutputPixel::Subpixel],
    ) -> Result<(), NokhwaError>;
}

// #[cfg(feature = "decoders")]
