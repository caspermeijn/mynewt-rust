/// Data mode of SPI driver, defined by HAL_SPI_MODEn.
pub enum DataMode {
    Mode0 = mynewt_core_hw_hal_bindgen::HAL_SPI_MODE0 as isize,
    Mode1 = mynewt_core_hw_hal_bindgen::HAL_SPI_MODE1 as isize,
    Mode2 = mynewt_core_hw_hal_bindgen::HAL_SPI_MODE2 as isize,
    Mode3 = mynewt_core_hw_hal_bindgen::HAL_SPI_MODE3 as isize,
}

/// Data order, either HAL_SPI_MSB_FIRST or HAL_SPI_LSB_FIRST.
pub enum DataOrder {
    MsbFirst = mynewt_core_hw_hal_bindgen::HAL_SPI_MSB_FIRST as isize,
    LsbFist = mynewt_core_hw_hal_bindgen::HAL_SPI_LSB_FIRST as isize,
}

/// The word size of the SPI transaction, either 8-bit or 9-bit.
pub enum WordSize {
    Size8bit = mynewt_core_hw_hal_bindgen::HAL_SPI_WORD_SIZE_8BIT as isize,
    Size9bit = mynewt_core_hw_hal_bindgen::HAL_SPI_WORD_SIZE_9BIT as isize,
}

/// Baudrate in kHz.
type Baudrate = u32;

pub struct Spi {
    num: i32,
    settings: mynewt_core_hw_hal_bindgen::hal_spi_settings,
}

impl Spi {
    pub unsafe fn new(num: i32) -> Spi {
        Spi {
            num,
            settings: mynewt_core_hw_hal_bindgen::hal_spi_settings {
                baudrate: 0,
                data_mode: 0,
                data_order: 0,
                word_size: 0,
            }
        }
    }

    pub fn config(&mut self, data_mode: DataMode, data_order: DataOrder, word_size: WordSize, baudrate: u32) -> Result<(), i32> {
        self.settings.data_mode = data_mode as u8;
        self.settings.data_order = data_order as u8;
        self.settings.word_size = word_size as u8;
        self.settings.baudrate = baudrate;

        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_spi_config(self.num, &mut self.settings) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn enable(&self) -> Result<(), i32> {
        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_spi_enable(self.num) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn tx(&self, tx_buf: &[u8]) -> Result<(), i32> {
        let rx_buf: [u8; 128] = [0; 128];
        assert!(tx_buf.len() < rx_buf.len());
        let length = tx_buf.len() as i32;
        let tx_buf = tx_buf.as_ptr() as *mut core::ffi::c_void;
        let rx_buf = rx_buf.as_ptr() as *mut core::ffi::c_void;
        let result = unsafe { mynewt_core_hw_hal_bindgen::hal_spi_txrx(self.num, tx_buf, rx_buf, length) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

