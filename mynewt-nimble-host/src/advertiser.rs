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

use core::convert::TryInto;

static mut BLE_ADVERTISER: BleAdvertiser = BleAdvertiser::default();

pub struct BleAdvertiser {
    own_addr_type: u8,
    adv_params: Option<mynewt_sys::ble_gap_adv_params>,
    fields: Option<mynewt_sys::ble_hs_adv_fields>,
}

impl BleAdvertiser {
    const fn default() -> BleAdvertiser {
        BleAdvertiser {
            own_addr_type: 0,
            adv_params: None,
            fields: None,
        }
    }

    pub fn start() {
        unsafe { assert!(mynewt_sys::ble_hs_cfg.sync_cb == None) };

        unsafe {
            mynewt_sys::ble_hs_cfg.sync_cb = Some(BleAdvertiser::bleprph_on_sync)
        };
    }

    fn bleprph_advertise() {
        let mut ble_settings = unsafe { &mut BLE_ADVERTISER };
        ble_settings.adv_params = Some(mynewt_sys::ble_gap_adv_params::default());
        ble_settings.fields = Some(mynewt_sys::ble_hs_adv_fields::default());

        let own_addr_type = &mut ble_settings.own_addr_type;
        let adv_params = ble_settings.adv_params.as_mut().unwrap();
        let fields = ble_settings.fields.as_mut().unwrap();

        let rc = unsafe { mynewt_sys::ble_hs_id_infer_auto(0, own_addr_type) };
        assert!(rc == 0);

        fields.flags = mynewt_sys::BLE_HS_ADV_F_DISC_GEN as u8
            | mynewt_sys::BLE_HS_ADV_F_BREDR_UNSUP as u8;

        fields.set_tx_pwr_lvl_is_present(1);
        fields.tx_pwr_lvl = mynewt_sys::BLE_HS_ADV_TX_PWR_LVL_AUTO as i8;

        // let name = mynewt_sys::ble_svc_gap_device_name();
        let name = "pinetime";
        fields.name = name.as_ptr();
        fields.name_len = name.len() as u8;
        fields.set_name_is_complete(1);

        let rc = unsafe { mynewt_sys::ble_gap_adv_set_fields(fields) };
        assert!(rc == 0);

        // /* Begin advertising. */
        adv_params.conn_mode = mynewt_sys::BLE_GAP_CONN_MODE_UND
            .try_into()
            .unwrap();
        adv_params.disc_mode = mynewt_sys::BLE_GAP_DISC_MODE_GEN
            .try_into()
            .unwrap();

        let rc = unsafe {
            mynewt_sys::ble_gap_adv_start(
                *own_addr_type,
                core::ptr::null_mut(),
                !0, /*BLE_HS_FOREVER*/
                adv_params,
                Some(BleAdvertiser::bleprph_gap_event),
                core::ptr::null_mut(),
            )
        };
        assert!(rc == 0);
    }

    unsafe extern "C" fn bleprph_gap_event(
        event: *mut mynewt_sys::ble_gap_event,
        arg: *mut cty::c_void,
    ) -> i32 {
        let event = (unsafe { *event });
        match event.type_ as u32 {
            mynewt_sys::BLE_GAP_EVENT_CONNECT => {
                if event.__bindgen_anon_1.connect.status != 0 {
                    /* Connection failed; resume advertising. */
                    BleAdvertiser::bleprph_advertise();
                }
            }
            mynewt_sys::BLE_GAP_EVENT_DISCONNECT => {
                /* Connection terminated; resume advertising. */
                BleAdvertiser::bleprph_advertise();
            }
            mynewt_sys::BLE_GAP_EVENT_DISCONNECT => {
                // advertise complete; reason=%d", event->adv_complete.reason);
                BleAdvertiser::bleprph_advertise();
            }
            _ => {}
        }
        return 0;
    }

    unsafe extern "C" fn bleprph_on_sync() {
        /* Make sure we have proper identity address set (public preferred) */
        let rc = mynewt_sys::ble_hs_util_ensure_addr(0);
        assert!(rc == 0);

        /* Begin advertising. */
        BleAdvertiser::bleprph_advertise();
    }
}
