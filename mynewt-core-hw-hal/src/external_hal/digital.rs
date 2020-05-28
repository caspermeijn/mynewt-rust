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

use crate::gpio::PinState::High;
use crate::gpio::PinState::Low;

impl embedded_hal::digital::v2::OutputPin for crate::gpio::OutputPin {
    type Error = !;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write(Low);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write(High);
        Ok(())
    }
}

impl embedded_hal::digital::v2::ToggleableOutputPin for crate::gpio::OutputPin {
    type Error = !;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}
