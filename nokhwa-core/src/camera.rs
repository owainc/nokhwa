use crate::control::{ControlDescription, ControlId, ControlValue, Controls};
use crate::error::NokhwaError;
use crate::frame_format::FrameFormat;
use crate::stream::StreamHandle;
use crate::types::{CameraFormat, FrameRate, Resolution};
use std::collections::hash_map::{Keys, Values};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Setting {
    fn enumerate_formats(&self) -> Result<Vec<CameraFormat>, NokhwaError>;

    fn enumerate_resolution_and_frame_rates(
        &self,
        frame_format: FrameFormat,
    ) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError>;

    fn set_format(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;

    fn control_ids(&self) -> Keys<ControlId, ControlDescription>;

    fn control_descriptions(&self) -> Values<ControlId, ControlDescription>;

    fn control_values(&self) -> Values<ControlId, ControlValue>;

    fn control_value(&self, id: &ControlId) -> Option<&ControlValue>;

    fn control_description(&self, id: &ControlId) -> Option<&ControlDescription>;

    fn set_control(&mut self, property: &ControlId, value: ControlValue)
        -> Result<(), NokhwaError>;

    fn refresh_controls(&mut self) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
pub trait AsyncSetting {
    async fn enumerate_formats_async(&self) -> Result<Vec<CameraFormat>, NokhwaError>;

    async fn enumerate_resolution_and_frame_rates_async(
        &self,
        frame_format: FrameFormat,
    ) -> Result<HashMap<Resolution, Vec<FrameRate>>, NokhwaError>;

    async fn set_format_async(&self, camera_format: CameraFormat) -> Result<(), NokhwaError>;

    async fn properties_async(&self) -> &Controls;

    async fn set_property_async(
        &mut self,
        property: &ControlId,
        value: ControlValue,
    ) -> Result<(), NokhwaError>;
}

pub trait Capture {
    // Implementations MUST guarantee that there can only ever be one stream open at once.
    fn open_stream(&mut self) -> Result<Arc<StreamHandle>, NokhwaError>;

    // Implementations MUST be multi-close tolerant.
    fn close_stream(&mut self) -> Result<(), NokhwaError>;
}

#[cfg(feature = "async")]
pub trait AsyncStream {
    async fn open_stream_async(&mut self) -> Result<StreamHandle, NokhwaError>;

    async fn close_stream_async(&mut self) -> Result<(), NokhwaError>;
}

pub trait Camera: Setting + Capture {}

#[cfg(feature = "async")]
pub trait AsyncCamera: Camera + AsyncSetting + AsyncStream {}
