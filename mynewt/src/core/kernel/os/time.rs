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

use alloc::boxed::Box;

pub struct Delay {}

pub struct TimeOfDay {
    time_val: mynewt_sys::os_timeval,
    timezone: mynewt_sys::os_timezone,
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
            time_val: mynewt_sys::os_timeval::default(),
            timezone: mynewt_sys::os_timezone::default(),
        };

        let result = unsafe {
            mynewt_sys::os_gettimeofday(&mut time.time_val, &mut time.timezone)
        };

        if result == 0 {
            Ok(time)
        } else {
            Err(result)
        }
    }

    fn get_local_timeval_seconds(&self) -> i64 {
        let local_offset =
            self.timezone.tz_minuteswest as i64 * 60 + self.timezone.tz_dsttime as i64 * 60;
        self.time_val.tv_sec - local_offset
    }
    pub fn hours_local(&self) -> u8 {
        (self.get_local_timeval_seconds() / 60 / 60 % 24) as u8
    }

    pub fn minutes_local(&self) -> u8 {
        (self.get_local_timeval_seconds() / 60 % 60) as u8
    }

    pub fn seconds_local(&self) -> u8 {
        (self.get_local_timeval_seconds() % 60) as u8
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

pub struct TimeChangeListener {
    listener: Option<mynewt_sys::os_time_change_listener>,
    closure: Option<Box<dyn FnMut() + Send + 'static>>,
}

impl TimeChangeListener {
    pub const fn new() -> TimeChangeListener {
        TimeChangeListener {
            listener: None,
            closure: None,
        }
    }

    pub fn register<F>(&'static mut self, mut func: F)
    where
        F: FnMut(),
        F: Send + 'static,
    {
        assert!(self.listener.is_none());

        self.listener = Some(mynewt_sys::os_time_change_listener::default());
        self.closure = Some(Box::new(func));

        self.listener.as_mut().unwrap().tcl_fn = Some(Self::callback::<F>);
        self.listener.as_mut().unwrap().tcl_arg =
            self.closure.as_mut().unwrap().as_mut() as *mut _ as *mut core::ffi::c_void;

        unsafe {
            mynewt_sys::os_time_change_listen(self.listener.as_mut().unwrap())
        };
    }

    unsafe extern "C" fn callback<F>(
        _: *const mynewt_sys::os_time_change_info,
        arg: *mut core::ffi::c_void,
    ) where
        F: FnMut(),
        F: Send + 'static,
    {
        let func_ptr = arg as *mut F;
        let func = &mut (*func_ptr);
        func();
    }
}
