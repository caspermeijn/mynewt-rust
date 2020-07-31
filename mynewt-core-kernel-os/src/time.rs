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

pub struct Delay {}

pub struct TimeOfDay {
    time_val: mynewt_core_kernel_os_bindgen::os_timeval,
    timezone: mynewt_core_kernel_os_bindgen::os_timezone,
}

impl TimeOfDay {
    /// int os_gettimeofday(struct os_timeval * utctime, struct os_timezone * tz)
    ///
    /// Get the current time of day.
    ///
    /// Returns the time of day in UTC into the tv argument, and returns the timezone (if set) into tz.
    ///
    /// Return
    /// 0 on success, non-zero on failure
    /// Parameters
    ///
    /// tv: The structure to put the UTC time of day into
    /// tz: The structure to put the timezone information into
    pub fn getTimeOfDay() -> Result<TimeOfDay, i32> {
        let mut time = TimeOfDay {
            time_val: mynewt_core_kernel_os_bindgen::os_timeval::default(),
            timezone: mynewt_core_kernel_os_bindgen::os_timezone::default(),
        };

        let result = unsafe {
            mynewt_core_kernel_os_bindgen::os_gettimeofday(&mut time.time_val, &mut time.timezone)
        };

        if result == 0 {
            Ok(time)
        } else {
            Err(result)
        }
    }

    pub fn hours(&self) -> u8 {
        (self.time_val.tv_sec / 60 / 60 % 24) as u8
    }

    pub fn minutes(&self) -> u8 {
        (self.time_val.tv_sec / 60 % 60) as u8
    }

    pub fn seconds(&self) -> u8 {
        (self.time_val.tv_sec % 60) as u8
    }
}
