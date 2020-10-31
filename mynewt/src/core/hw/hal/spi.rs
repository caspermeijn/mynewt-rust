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

/// Data mode of SPI driver, defined by HAL_SPI_MODEn.
pub enum DataMode {
    Mode0 = mynewt_sys::HAL_SPI_MODE0 as isize,
    Mode1 = mynewt_sys::HAL_SPI_MODE1 as isize,
    Mode2 = mynewt_sys::HAL_SPI_MODE2 as isize,
    Mode3 = mynewt_sys::HAL_SPI_MODE3 as isize,
}

/// Data order, either HAL_SPI_MSB_FIRST or HAL_SPI_LSB_FIRST.
pub enum DataOrder {
    MsbFirst = mynewt_sys::HAL_SPI_MSB_FIRST as isize,
    LsbFist = mynewt_sys::HAL_SPI_LSB_FIRST as isize,
}

/// The word size of the SPI transaction, either 8-bit or 9-bit.
pub enum WordSize {
    Size8bit = mynewt_sys::HAL_SPI_WORD_SIZE_8BIT as isize,
    Size9bit = mynewt_sys::HAL_SPI_WORD_SIZE_9BIT as isize,
}

/// Baudrate in kHz.
type Baudrate = u32;

pub struct Spi {
    num: i32,
    settings: mynewt_sys::hal_spi_settings,
}

impl Spi {
    pub unsafe fn new(num: i32) -> Spi {
        Spi {
            num,
            settings: mynewt_sys::hal_spi_settings {
                baudrate: 0,
                data_mode: 0,
                data_order: 0,
                word_size: 0,
            },
        }
    }

    pub fn config(
        &mut self,
        data_mode: DataMode,
        data_order: DataOrder,
        word_size: WordSize,
        baudrate: Baudrate,
    ) -> Result<(), i32> {
        self.settings.data_mode = data_mode as u8;
        self.settings.data_order = data_order as u8;
        self.settings.word_size = word_size as u8;
        self.settings.baudrate = baudrate;

        let result = unsafe { mynewt_sys::hal_spi_config(self.num, &mut self.settings) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn enable(&self) -> Result<(), i32> {
        let result = unsafe { mynewt_sys::hal_spi_enable(self.num) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn tx(&self, tx_buf: &[u8]) -> Result<(), i32> {
        let rx_buf: [u8; 128] = [0; 128];
        assert!(tx_buf.len() <= rx_buf.len());
        let length = tx_buf.len() as i32;
        let tx_buf = tx_buf.as_ptr() as *mut core::ffi::c_void;
        let rx_buf = rx_buf.as_ptr() as *mut core::ffi::c_void;
        let result = unsafe { mynewt_sys::hal_spi_txrx(self.num, tx_buf, rx_buf, length) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}
