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
use mynewt_core_hw_hal::spi::{DataMode, DataOrder, Spi, WordSize};
use mynewt_core_kernel_os::time::Delay;

mod binding;

pub struct Bsp {
    pub backlight_low: Option<OutputPin>,
    pub backlight_medium: Option<OutputPin>,
    pub backlight_high: Option<OutputPin>,
    pub display_chip_select: Option<OutputPin>,
    pub display_data_command: Option<OutputPin>,
    pub display_reset: Option<OutputPin>,
    pub spi: Option<Spi>,
}

impl Bsp {
    pub const fn new() -> Bsp {
        Bsp {
            backlight_low: None,
            backlight_medium: None,
            backlight_high: None,
            display_chip_select: None,
            display_data_command: None,
            display_reset: None,
            spi: None,
        }
    }

    pub fn init(&mut self) {
        let backlight_low_pin = unsafe { Gpio::new(binding::LCD_BACKLIGHT_LOW_PIN as i32) };
        let backlight_medium_pin = unsafe { Gpio::new(binding::LCD_BACKLIGHT_MED_PIN as i32) };
        let backlight_high_pin = unsafe { Gpio::new(binding::LCD_BACKLIGHT_HIGH_PIN as i32) };

        let display_chip_select_pin = unsafe { Gpio::new(binding::LCD_CHIP_SELECT_PIN as i32) };
        let display_data_command_pin = unsafe { Gpio::new(binding::LCD_WRITE_PIN as i32) };
        let display_reset_pin = unsafe { Gpio::new(binding::LCD_RESET_PIN as i32) };

        let mut spi = unsafe { Spi::new(0) };

        spi.config(
            DataMode::Mode3,
            DataOrder::MsbFirst,
            WordSize::Size8bit,
            8000,
        );

        self.backlight_low = Some(backlight_low_pin.init_as_output().unwrap());
        self.backlight_medium = Some(backlight_medium_pin.init_as_output().unwrap());
        self.backlight_high = Some(backlight_high_pin.init_as_output().unwrap());
        self.display_chip_select = Some(display_chip_select_pin.init_as_output().unwrap());
        self.display_data_command = Some(display_data_command_pin.init_as_output().unwrap());
        self.display_reset = Some(display_reset_pin.init_as_output().unwrap());
        self.spi = Some(spi);
    }
}
