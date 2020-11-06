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

//! This module provides a generic BSP, which will be applicable as a placeholder for example
//! applications. For example the blinky application uses this.

use crate::core::hw::hal::gpio::{Gpio, OutputPin};
use spin::Mutex;

static IS_TAKEN: Mutex<bool> = Mutex::new(false);

pub struct Bsp {
    pub led_blink: OutputPin,
}

impl Bsp {
    pub fn take() -> Option<Bsp> {
        let is_taken = IS_TAKEN.lock();
        if !*is_taken {
            Some(Self::init())
        } else {
            None
        }
    }

    pub unsafe fn steal() -> Bsp {
        *(IS_TAKEN.lock()) = true;
        Self::init()
    }

    fn init() -> Bsp {
        let led_blink_pin = unsafe { Gpio::new(mynewt_sys::LED_BLINK_PIN as i32) };
        Bsp {
            led_blink: led_blink_pin.init_as_output().unwrap(),
        }
    }
}
