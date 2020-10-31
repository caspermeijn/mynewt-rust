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

use core::alloc::{GlobalAlloc, Layout};

struct MynewtAllocator;

unsafe impl GlobalAlloc for MynewtAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        mynewt_sys::os_malloc(layout.size() as cty::c_uint) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mynewt_sys::os_free(ptr as *mut cty::c_void)
    }
}

#[global_allocator]
static A: MynewtAllocator = MynewtAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
