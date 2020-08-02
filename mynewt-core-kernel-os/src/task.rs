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
use core::convert::TryInto;

pub struct Task {
    task: Option<mynewt_core_kernel_os_bindgen::os_task>,
    stack: [mynewt_core_kernel_os_bindgen::os_stack_t; 4000],
    closure: Option<Box<FnMut() + Send + 'static>>,
}

impl Task {
    pub const fn new() -> Task {
        Task {
            task: None,
            stack: [0; 4000],
            closure: None,
        }
    }

    pub fn init<F>(&'static mut self, name: &'static str, mut func: F, prio: u8) -> Result<(), i32>
    where
        F: FnMut(),
        F: Send + 'static,
    {
        self.task = Some(mynewt_core_kernel_os_bindgen::os_task::default());
        self.closure = Some(Box::new(func));

        let task = self.task.as_mut().unwrap();
        let name = name.as_ptr() as *const cty::c_char;
        let callback = Task::task_callback::<F>;
        let arg = self.closure.as_mut().unwrap().as_mut() as *mut _ as *mut core::ffi::c_void;
        let stack_bottom = self.stack.as_mut_ptr();
        let stack_size = self.stack.len().try_into().unwrap();

        let result = unsafe {
            mynewt_core_kernel_os_bindgen::os_task_init(
                task,
                name,
                Some(callback),
                arg,
                prio,
                !0, //TODO: mynewt_core_kernel_os_bindgen::OS_WAIT_FOREVER,
                stack_bottom,
                stack_size,
            )
        };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    unsafe extern "C" fn task_callback<F>(arg: *mut cty::c_void)
    where
        F: FnMut(),
        F: Send + 'static,
    {
        let func_ptr = arg as *mut F;
        let func = unsafe { &mut *func_ptr };
        func();
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        unimplemented!()
    }
}
