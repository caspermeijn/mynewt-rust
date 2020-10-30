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

use mynewt_core_hw_bus::spi::SpiNode;
use mynewt_core_hw_hal::gpio::{Gpio, OutputPin};
use mynewt_core_hw_hal::spi::{DataMode, DataOrder, WordSize};
use mynewt_core_kernel_os::time::Delay;

pub struct Bsp {
    pub backlight_low: Option<OutputPin>,
    pub backlight_medium: Option<OutputPin>,
    pub backlight_high: Option<OutputPin>,
    pub display_data_command: Option<OutputPin>,
    pub display_reset: Option<OutputPin>,
    pub display_spi: Option<SpiNode>,
}

impl Bsp {
    pub const fn new() -> Bsp {
        Bsp {
            backlight_low: None,
            backlight_medium: None,
            backlight_high: None,
            display_data_command: None,
            display_reset: None,
            display_spi: None,
        }
    }

    pub fn init(&'static mut self) {
        let backlight_low_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_LOW_PIN as i32) };
        let backlight_medium_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_MED_PIN as i32) };
        let backlight_high_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_HIGH_PIN as i32) };

        let display_data_command_pin = unsafe { Gpio::new(mynewt_sys::LCD_WRITE_PIN as i32) };
        let display_reset_pin = unsafe { Gpio::new(mynewt_sys::LCD_RESET_PIN as i32) };
        let mut display_spi = unsafe { SpiNode::new("spidisplay\0") };

        display_spi.open();

        self.backlight_low = Some(backlight_low_pin.init_as_output().unwrap());
        self.backlight_medium = Some(backlight_medium_pin.init_as_output().unwrap());
        self.backlight_high = Some(backlight_high_pin.init_as_output().unwrap());
        self.display_data_command = Some(display_data_command_pin.init_as_output().unwrap());
        self.display_reset = Some(display_reset_pin.init_as_output().unwrap());
        self.display_spi = Some(display_spi);
    }
}
