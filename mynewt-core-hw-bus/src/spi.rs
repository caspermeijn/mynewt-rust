use alloc::boxed::Box;
use core::marker::PhantomPinned;
use core::pin::Pin;
use cortex_m_semihosting::hprintln;
use mynewt_core_hw_hal::spi::DataMode;
use mynewt_core_hw_hal::spi::DataOrder;

pub struct SpiNode {
    dev_name: &'static str,
    os_dev: *mut mynewt_core_hw_bus_bindgen::os_dev,
}

unsafe impl Send for SpiNode {}

impl SpiNode {
    pub unsafe fn new(dev_name: &'static str) -> Self {
        assert_eq!(dev_name.chars().last().unwrap(), '\0');

        SpiNode {
            dev_name,
            os_dev: core::ptr::null_mut(),
        }
    }

    pub fn open(&mut self) {
        let os_dev = unsafe {
            mynewt_core_hw_bus_bindgen::os_dev_open(
                self.dev_name.as_ptr(),
                0,
                core::ptr::null_mut(),
            )
        };
        assert!(os_dev != core::ptr::null_mut());

        self.os_dev = os_dev;
    }

    pub fn write(&mut self, tx_buf: &[u8]) -> Result<(), i32> {
        assert!(self.os_dev != core::ptr::null_mut());

        let timeout = mynewt_core_hw_bus_bindgen::MYNEWT_VAL_BUS_DEFAULT_TRANSACTION_TIMEOUT_MS;
        let mut ticks: mynewt_core_kernel_os_bindgen::os_time_t = 0;
        let result =
            unsafe { mynewt_core_kernel_os_bindgen::os_time_ms_to_ticks(timeout, &mut ticks) };
        assert!(result == 0);

        let result = unsafe {
            mynewt_core_hw_bus_bindgen::bus_node_write(
                self.os_dev,
                tx_buf.as_ptr() as *const core::ffi::c_void,
                tx_buf.len() as u16,
                ticks,
                mynewt_core_hw_bus_bindgen::BUS_F_NONE as u16,
            )
        };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}
