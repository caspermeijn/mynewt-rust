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

use mynewt_core_hw_hal::gpio::{Gpio, OutputPin};

mod binding;

pub struct Bsp {
    pub backlight_low: OutputPin,
    pub backlight_medium: OutputPin,
    pub backlight_high: OutputPin,
}

impl Bsp {
    pub fn new() -> Bsp {
        let backlight_low_pin = unsafe{Gpio::new(binding::LCD_BACKLIGHT_LOW_PIN as i32)};
        let backlight_medium_pin = unsafe{Gpio::new(binding::LCD_BACKLIGHT_MED_PIN as i32)};
        let backlight_high_pin = unsafe{Gpio::new(binding::LCD_BACKLIGHT_HIGH_PIN as i32)};
        Bsp {
            backlight_low: backlight_low_pin.init_as_output().unwrap(),
            backlight_medium: backlight_medium_pin.init_as_output().unwrap(),
            backlight_high: backlight_high_pin.init_as_output().unwrap(),
        }
    }
}
