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

pub struct SpiNode {
    dev_name: &'static str,
    os_dev: *mut mynewt_sys::os_dev,
}

unsafe impl Send for SpiNode {}

impl SpiNode {
    pub unsafe fn new(dev_name: &'static str) -> Self {
        assert_eq!(dev_name.chars().last().unwrap(), '\0');

        SpiNode {
            dev_name,
            os_dev: core::ptr::null_mut(),
        }
    }

    pub fn open(&mut self) {
        let os_dev =
            unsafe { mynewt_sys::os_dev_open(self.dev_name.as_ptr(), 0, core::ptr::null_mut()) };
        assert!(os_dev != core::ptr::null_mut());

        self.os_dev = os_dev;
    }

    pub fn write(&mut self, tx_buf: &[u8]) -> Result<(), i32> {
        assert!(self.os_dev != core::ptr::null_mut());

        let timeout = mynewt_sys::MYNEWT_VAL_BUS_DEFAULT_TRANSACTION_TIMEOUT_MS;
        let mut ticks: mynewt_sys::os_time_t = 0;
        let result = unsafe { mynewt_sys::os_time_ms_to_ticks(timeout, &mut ticks) };
        assert!(result == 0);

        let result = unsafe {
            mynewt_sys::bus_node_write(
                self.os_dev,
                tx_buf.as_ptr() as *const core::ffi::c_void,
                tx_buf.len() as u16,
                ticks,
                mynewt_sys::BUS_F_NONE as u16,
            )
        };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}
