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

extern crate mynewt_bindgen_helper;

fn main() {
    let header_files = vec![
        // "../apache-mynewt-nimble/nimble/host/include/host/ble_att.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_eddystone.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_gap.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_gatt.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_adv.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_hci.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_id.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_log.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_mbuf.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_hs_stop.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_ibeacon.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_l2cap.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_monitor.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_sm.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_store.h",
        "../apache-mynewt-nimble/nimble/host/include/host/ble_uuid.h",
        "../apache-mynewt-nimble/nimble/host/util/include/host/util/util.h",
    ];

    let result = mynewt_bindgen_helper::generate(header_files);
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
