#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![cfg_attr(feature = "test-fail-warning", deny(warnings))]
#![cfg_attr(feature = "docs-features", feature(doc_cfg))]
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

//! Core type definitions for `nokhwa`
pub mod camera;
pub mod decoder;
pub mod error;
pub mod format_request;
pub mod frame_buffer;
pub mod frame_format;
pub mod control;
pub mod ranges;
pub mod traits;
pub mod types;
pub mod utils;
pub mod stream;
pub mod platform;
