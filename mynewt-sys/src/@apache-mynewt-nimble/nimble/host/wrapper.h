/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: CC0-1.0
 */
#include <inttypes.h>
#include "host/ble_att.h"
#include "host/ble_eddystone.h"
#include "host/ble_gap.h"
#include "host/ble_gatt.h"
#include "host/ble_hs.h"
#include "host/ble_hs_adv.h"
#include "host/ble_hs_hci.h"
#include "host/ble_hs_id.h"
#include "host/ble_hs_log.h"
#include "host/ble_hs_mbuf.h"
#include "host/ble_hs_stop.h"
#include "host/ble_ibeacon.h"
#include "host/ble_l2cap.h"
#include "host/ble_monitor.h"
#include "host/ble_sm.h"
#include "host/ble_store.h"
#include "host/ble_uuid.h"
