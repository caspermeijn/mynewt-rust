/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2021 Casper Meijn <casper@meijn.net>
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

use mynewt_sys;

pub struct ChargeControl {
    charger: &'static mut mynewt_sys::charge_control,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Type {
    /// No type, used for queries
    None = mynewt_sys::charge_control_type_t_CHARGE_CONTROL_TYPE_NONE as isize,
    /// Charging status reporting supported
    Status = mynewt_sys::charge_control_type_t_CHARGE_CONTROL_TYPE_STATUS as isize,
    /// Fault reporting supported
    Fault = mynewt_sys::charge_control_type_t_CHARGE_CONTROL_TYPE_FAULT as isize,
}

/// Possible charge controller states
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Status {
    /// Charge controller is disabled (if enable/disable function exists)
    Disabled = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_DISABLED as isize,
    /// No charge source is present at the charge controller input
    NoSource = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_NO_SOURCE as isize,
    /// Charge controller is charging a battery
    Charging = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_CHARGING as isize,
    /// Charge controller has completed its charging cycle
    ChargeComplete =
        mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_CHARGE_COMPLETE as isize,
    /// Charging is temporarily suspended
    Suspend = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_SUSPEND as isize,
    /// Charge controller has detected a fault condition
    Fault = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_FAULT as isize,
    /// Unspecified status; User must understand how to interpret
    Other = mynewt_sys::charge_control_status_t_CHARGE_CONTROL_STATUS_OTHER as isize,
}

impl ChargeControl {
    pub fn find_first_by_type(type_: Type) -> Result<ChargeControl, ()> {
        let charger = unsafe {
            mynewt_sys::charge_control_mgr_find_next_bytype(type_ as u8, core::ptr::null_mut())
        };
        if charger != core::ptr::null_mut() {
            Ok(ChargeControl {
                charger: unsafe { (charger as *mut mynewt_sys::charge_control).as_mut() }.unwrap(),
            })
        } else {
            Err(())
        }
    }

    pub fn read_status_blocking(&mut self) -> Status {
        let mut result = Status::ChargeComplete;

        let rc = unsafe {
            mynewt_sys::charge_control_read(
                self.charger,
                Type::Status as u8,
                Some(Self::read_status_callback),
                &mut result as *mut _ as *mut core::ffi::c_void,
                1000000,
            )
        };
        assert_eq!(rc, 0);

        result
    }

    unsafe extern "C" fn read_status_callback(
        _: *mut mynewt_sys::charge_control,
        arg: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void,
        type_: u8,
    ) -> i32 {
        let arg = arg as *mut Status;
        let data = data as *mut Status;
        assert_eq!(type_, Type::Status as u8);

        *arg = *data;

        0
    }
}
