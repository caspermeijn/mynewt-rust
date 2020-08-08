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

use super::embedded_hal::blocking::spi::Write;
use crate::spi::SpiNode;

impl embedded_hal::blocking::spi::Write<u8> for SpiNode {
    type Error = i32;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.write(words)
    }
}
