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

#![no_std]

extern crate panic_halt;

use mynewt::core::hw::bsp::generic::Bsp;
use mynewt::core::hw::hal::gpio::PinState;
use mynewt::core::kernel::os::time::delay_milliseconds;

#[no_mangle]
pub extern "C" fn main() {
    let bsp = Bsp::take().unwrap();
    let mut led_blink = bsp.led_blink;

    /* Initialize all packages. */
    unsafe {
        mynewt_sys::sysinit_start();
        mynewt_sys::sysinit_app();
        mynewt_sys::sysinit_end();
    }

    /* Turn on the LED */
    led_blink.write(PinState::High);

    loop {
        /* Wait one second */
        delay_milliseconds(1000);

        /* Toggle the LED */
        led_blink.toggle();
    }
}
