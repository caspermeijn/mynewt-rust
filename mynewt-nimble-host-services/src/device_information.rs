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

pub fn set_model_number(model_number: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_model_number_set(
            model_number.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_serial_number(serial_number: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_serial_number_set(
            serial_number.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_firmware_revision(firmware_revision: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_firmware_revision_set(
            firmware_revision.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_hardware_revision(hardware_revision: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_hardware_revision_set(
            hardware_revision.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_software_revision(software_revision: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_software_revision_set(
            software_revision.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_manufacturer_name(manufacturer_name: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_manufacturer_name_set(
            manufacturer_name.as_ptr(),
        );
        assert!(rc == 0);
    }
}

pub fn set_system_id(system_id: &'static str) {
    unsafe {
        let rc = mynewt_nimble_host_services_bindgen::ble_svc_dis_system_id_set(system_id.as_ptr());
        assert!(rc == 0);
    }
}
