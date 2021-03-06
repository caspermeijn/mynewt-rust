/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2020 Casper Meijn <casper@meijn.net>
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

extern crate alloc;

use alloc::format;
use alloc::string::String;

#[derive(Default)]
pub struct ImageVersion {
    raw_version: mynewt_sys::image_version,
}

impl ImageVersion {
    pub fn get_current() -> Option<Self> {
        let mut version = ImageVersion::default();
        let rc = unsafe { mynewt_sys::imgr_my_version(&mut version.raw_version) };

        if rc == 0 {
            Some(version)
        } else {
            None
        }
    }
}

impl From<ImageVersion> for String {
    fn from(version: ImageVersion) -> Self {
        format!(
            "{}.{}.{}",
            version.raw_version.iv_major,
            version.raw_version.iv_minor,
            version.raw_version.iv_revision
        )
    }
}
