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

pub fn loop_default_queue() -> ! {
    loop {
        unsafe {
            mynewt_core_kernel_os_bindgen::os_eventq_run(
                mynewt_core_kernel_os_bindgen::os_eventq_dflt_get(),
            );
        }
    }
}

pub struct EventQueue {
    event_queue: Option<mynewt_core_kernel_os_bindgen::os_eventq>,
}

impl EventQueue {
    pub const fn new() -> EventQueue {
        EventQueue {
            event_queue: None,
        }
    }

    pub fn init(&'static mut self) {
        assert!(self.event_queue.is_none());
        self.event_queue = Some(mynewt_core_kernel_os_bindgen::os_eventq::default());
        unsafe { mynewt_core_kernel_os_bindgen::os_eventq_init(self.event_queue.as_mut().unwrap()) };
    }

    pub fn run(&'static mut self) {
        assert!(self.event_queue.is_some());
        unsafe { mynewt_core_kernel_os_bindgen::os_eventq_run(self.event_queue.as_mut().unwrap()) };
    }

    pub unsafe fn as_raw_mut(&'static mut self) -> &'static mut mynewt_core_kernel_os_bindgen::os_eventq {
        self.event_queue.as_mut().unwrap()
    }
}
