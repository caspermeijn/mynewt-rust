/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: CC0-1.0
 */

#include "sysinit/sysinit.h"
#ifdef ARCH_sim
#include "mcu/mcu_sim.h"
#endif

void shim_sysinit() {
    sysinit();
}

void shim_sim_init() {
#ifdef ARCH_sim
    int argc = 0;
    char **argv  = {"dummy"};
    mcu_sim_parse_args(argc, argv);
#endif
}
