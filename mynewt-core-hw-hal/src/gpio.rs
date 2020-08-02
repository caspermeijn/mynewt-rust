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

//! This is the hardware independent GPIO (General Purpose Input Output) Interface for Mynewt.
//! See https://mynewt.apache.org/latest/os/modules/hal/hal_gpio/hal_gpio.html for more details.
//!
//! An application should not create instances of Gpio. These should be obtained from an BSP crate.
//!
//! An GPIO can be initialized to an OutputPin. This can be written to set the output state.
//! Currently only the output is implemented

pub enum PinState {
    Low,
    High,
}

impl From<i32> for PinState {
    fn from(item: i32) -> Self {
        if item == 0 {
            PinState::Low
        } else {
            PinState::High
        }
    }
}

impl PinState {
    fn as_i32(&self) -> i32 {
        match self {
            PinState::Low => 0,
            PinState::High => 1,
        }
    }
}

pub struct Gpio {
    pin: i32,
}

impl Gpio {
    /// Creates a new instance op Gpio. This should not be done by an opplication, but by a BSP.
    /// This call is unsafe as an incorrect pin number could access unwanted memory.
    pub unsafe fn new(pin: i32) -> Gpio {
        Gpio { pin }
    }

    /// Initialize the GPIO as an output.
    pub fn init_as_output(self) -> Result<OutputPin, i32> {
        OutputPin::new(self)
    }
}

pub struct OutputPin {
    gpio: Gpio,
}

impl OutputPin {
    fn new(gpio: Gpio) -> Result<OutputPin, i32> {
        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_gpio_init_out(gpio.pin, 0) };
        if result == 0 {
            Ok(OutputPin { gpio })
        } else {
            Err(result)
        }
    }

    /// Deinitialize the GPIO to revert the previous initialization.
    pub fn deinit(self) -> Result<Gpio, i32> {
        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_gpio_deinit(self.gpio.pin) };
        if result == 0 {
            Ok(self.gpio)
        } else {
            Err(result)
        }
    }

    /// Write a value (either high or low) to the specified pin.
    pub fn write(&mut self, value: PinState) {
        unsafe {
            mynewt_core_hw_hal_bindgen::hal_gpio_write(self.gpio.pin, value.as_i32());
        }
    }

    /// Toggles the specified pin.
    /// Returns: current gpio state
    pub fn toggle(&mut self) -> PinState {
        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_gpio_toggle(self.gpio.pin) };
        PinState::from(result)
    }
}
