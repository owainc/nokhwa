/*
 * Copyright 2022 l1npengtul <l1npengtul@protonmail.com> / The Nokhwa Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::hash::{Hash, Hasher};
use crate::frame_format::FrameFormat;
use crate::types::Resolution;
use small_map::{FxSmallMap, Iter};
use crate::control::ControlValue;

pub type PlatformSpecificFlag = u32;

#[derive(Clone, Debug, Default)]
pub struct Metadata {
    flags: FxSmallMap<8, u32, ControlValue>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            flags: Default::default(),
        }
    } 
    
    pub fn get(&self, key: u32) -> Option<&ControlValue> {
        self.flags.get(&key)
    }
    
    pub fn insert(&mut self, key: u32, value: ControlValue) {
        self.flags.insert(key, value);
    }
    
    pub fn iter(&self) -> Iter<'_, 8, u32, ControlValue> {
        self.flags.iter()
    }
}

impl Hash for Metadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.flags {
            state.write_u32(key);
            value.hash(state);
        }
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        for (this_key, this_value) in &self.flags {
            if let Some(other_value) = other.flags.get(this_key) {
                if this_value != other_value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

/// A buffer returned by a camera to accommodate custom decoding.
/// Contains information of Resolution, the buffer's [`FrameFormat`], and the buffer.
///
/// Note that decoding on the main thread **will** decrease your performance and lead to dropped frames.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct FrameBuffer {
    resolution: Resolution,
    buffer: Vec<u8>,
    source_frame_format: FrameFormat,
    metadata: Option<Metadata>,
}

impl FrameBuffer {
    /// Creates a new buffer with a [`&[u8]`].
    #[must_use]
    #[inline]
    pub fn new(resolution: Resolution, buffer: Vec<u8>, source_frame_format: FrameFormat, metadata: Option<Metadata>) -> Self {
        Self {
            resolution,
            buffer,
            source_frame_format,
            metadata,
        }
    }

    /// Get the [`Resolution`] of this buffer.
    #[must_use]
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// Get the data of this buffer.
    #[must_use]
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    #[must_use]
    pub fn consume(self) -> Vec<u8> {
        self.buffer
    }
    
    #[must_use]
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Get the [`SourceFrameFormat`] of this buffer.
    #[must_use]
    pub fn source_frame_format(&self) -> FrameFormat {
        self.source_frame_format
    }
}
