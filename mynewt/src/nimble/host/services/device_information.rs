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

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;

fn get_leaked_null_terminated_string(input: String) -> &'static String {
    let null_terminated = if input.ends_with("\0") {
        input
    } else {
        input + "\0"
    };
    Box::leak(Box::new(null_terminated))
}

pub fn set_model_number(model_number: String) {
    let model_number: &'static String = get_leaked_null_terminated_string(model_number);

    let rc = unsafe { mynewt_sys::ble_svc_dis_model_number_set(model_number.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_serial_number(serial_number: String) {
    let serial_number: &'static String = get_leaked_null_terminated_string(serial_number);

    let rc = unsafe { mynewt_sys::ble_svc_dis_serial_number_set(serial_number.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_firmware_revision(firmware_revision: String) {
    let firmware_revision: &'static String = get_leaked_null_terminated_string(firmware_revision);

    let rc = unsafe { mynewt_sys::ble_svc_dis_firmware_revision_set(firmware_revision.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_hardware_revision(hardware_revision: String) {
    let hardware_revision: &'static String = get_leaked_null_terminated_string(hardware_revision);

    let rc = unsafe { mynewt_sys::ble_svc_dis_hardware_revision_set(hardware_revision.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_software_revision(software_revision: String) {
    let software_revision: &'static String = get_leaked_null_terminated_string(software_revision);

    let rc = unsafe { mynewt_sys::ble_svc_dis_software_revision_set(software_revision.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_manufacturer_name(manufacturer_name: String) {
    let manufacturer_name: &'static String = get_leaked_null_terminated_string(manufacturer_name);

    let rc = unsafe { mynewt_sys::ble_svc_dis_manufacturer_name_set(manufacturer_name.as_ptr()) };
    assert!(rc == 0);
}

pub fn set_system_id(system_id: String) {
    let system_id: &'static String = get_leaked_null_terminated_string(system_id);

    let rc = unsafe { mynewt_sys::ble_svc_dis_system_id_set(system_id.as_ptr()) };
    assert!(rc == 0);
}
