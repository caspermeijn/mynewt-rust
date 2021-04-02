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

use crate::core::hw::bus::spi::SpiNode;
use crate::core::hw::hal::gpio::{Gpio, OutputPin};
use embedded_hal::digital::v2::OutputPin as _;
use spin::Mutex;

static IS_TAKEN: Mutex<bool> = Mutex::new(false);

pub struct Backlight {
    pub backlight_low: OutputPin,
    pub backlight_medium: OutputPin,
    pub backlight_high: OutputPin,
}

impl Backlight {
    pub fn set_percentage(&mut self, percentage: u8) {
        if percentage == 0 {
            self.backlight_low.set_high().unwrap();
            self.backlight_medium.set_high().unwrap();
            self.backlight_high.set_high().unwrap();
        } else if percentage <= 33 {
            self.backlight_low.set_low().unwrap();
            self.backlight_medium.set_high().unwrap();
            self.backlight_high.set_high().unwrap();
        } else if percentage <= 66 {
            self.backlight_low.set_low().unwrap();
            self.backlight_medium.set_low().unwrap();
            self.backlight_high.set_high().unwrap();
        } else {
            self.backlight_low.set_low().unwrap();
            self.backlight_medium.set_low().unwrap();
            self.backlight_high.set_low().unwrap();
        }
    }
}

pub struct Bsp {
    pub backlight: Backlight,
    pub display_data_command: OutputPin,
    pub display_reset: OutputPin,
    pub display_spi: SpiNode,
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
        let backlight_low_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_LOW_PIN as i32) };
        let backlight_medium_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_MED_PIN as i32) };
        let backlight_high_pin = unsafe { Gpio::new(mynewt_sys::LCD_BACKLIGHT_HIGH_PIN as i32) };

        let display_data_command_pin = unsafe { Gpio::new(mynewt_sys::LCD_WRITE_PIN as i32) };
        let display_reset_pin = unsafe { Gpio::new(mynewt_sys::LCD_RESET_PIN as i32) };
        let mut display_spi = unsafe { SpiNode::new("spidisplay\0") };

        display_spi.open();

        Bsp {
            backlight: Backlight {
                backlight_low: backlight_low_pin.init_as_output().unwrap(),
                backlight_medium: backlight_medium_pin.init_as_output().unwrap(),
                backlight_high: backlight_high_pin.init_as_output().unwrap(),
            },
            display_data_command: display_data_command_pin.init_as_output().unwrap(),
            display_reset: display_reset_pin.init_as_output().unwrap(),
            display_spi,
        }
    }
}
