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

use crate::core::kernel::os::queue::EventQueue;
use alloc::boxed::Box;

pub struct Callout {
    callout: Option<mynewt_sys::os_callout>,
    closure: Option<Box<dyn FnMut() + Send + 'static>>,
}

impl Callout {
    pub const fn new() -> Callout {
        Callout {
            callout: None,
            closure: None,
        }
    }

    pub fn init<F>(&'static mut self, mut func: F, event_queue: &'static mut EventQueue)
    where
        F: FnMut(),
        F: Send + 'static,
    {
        self.callout = Some(mynewt_sys::os_callout::default());
        self.closure = Some(Box::new(func));

        let arg = self.closure.as_mut().unwrap().as_mut() as *mut _ as *mut core::ffi::c_void;

        unsafe {
            mynewt_sys::os_callout_init(
                self.callout.as_mut().unwrap(),
                event_queue.as_raw_mut(),
                Some(Callout::callback::<F>),
                arg,
            )
        };
    }

    pub fn init_default_queue<F>(&'static mut self, mut func: F)
    where
        F: FnMut(),
        F: Send + 'static,
    {
        self.callout = Some(mynewt_sys::os_callout::default());
        self.closure = Some(Box::new(func));

        let arg = self.closure.as_mut().unwrap().as_mut() as *mut _ as *mut core::ffi::c_void;

        unsafe {
            mynewt_sys::os_callout_init(
                self.callout.as_mut().unwrap(),
                mynewt_sys::os_eventq_dflt_get(),
                Some(Callout::callback::<F>),
                arg,
            )
        };
    }

    pub fn reset(&mut self, delay_ms: u32) {
        let mut ticks: mynewt_sys::os_time_t = 0;
        let result = unsafe { mynewt_sys::os_time_ms_to_ticks(delay_ms, &mut ticks) };
        assert!(result == 0);

        let result = unsafe { mynewt_sys::os_callout_reset(self.callout.as_mut().unwrap(), ticks) };
        assert!(result == 0);
    }

    unsafe extern "C" fn callback<F>(arg: *mut mynewt_sys::os_event)
    where
        F: FnMut(),
        F: Send + 'static,
    {
        let func_ptr = (*arg).ev_arg as *mut F;
        let func = &mut *func_ptr;
        func();
    }
}
