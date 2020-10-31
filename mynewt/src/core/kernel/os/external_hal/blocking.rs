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

use crate::core::kernel::os::time::Delay;

impl embedded_hal::blocking::delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        let mut ticks: mynewt_sys::os_time_t = 0;
        let result = unsafe { mynewt_sys::os_time_ms_to_ticks(ms, &mut ticks) };
        assert!(result == 0);
        unsafe { mynewt_sys::os_time_delay(ticks) };
    }
}

impl embedded_hal::blocking::delay::DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        unsafe { mynewt_sys::os_cputime_delay_usecs(us) };
    }
}
