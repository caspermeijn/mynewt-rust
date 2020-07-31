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
        "kernel/os/include/os/endian.h",
        "kernel/os/include/os/mynewt.h",
        "kernel/os/include/os/os.h",
        "kernel/os/include/os/os_callout.h",
        "kernel/os/include/os/os_cfg.h",
        "kernel/os/include/os/os_cputime.h",
        "kernel/os/include/os/os_dev.h",
        "kernel/os/include/os/os_error.h",
        "kernel/os/include/os/os_eventq.h",
        "kernel/os/include/os/os_fault.h",
        "kernel/os/include/os/os_heap.h",
        "kernel/os/include/os/os_malloc.h",
        "kernel/os/include/os/os_mbuf.h",
        "kernel/os/include/os/os_mempool.h",
        "kernel/os/include/os/os_mutex.h",
        "kernel/os/include/os/os_sanity.h",
        "kernel/os/include/os/os_sched.h",
        "kernel/os/include/os/os_sem.h",
        "kernel/os/include/os/os_task.h",
        "kernel/os/include/os/os_time.h",
        "kernel/os/include/os/os_trace_api.h",
        "kernel/os/include/os/queue.h",
        "kernel/os/include/os/util.h",
    ];

    let result = mynewt_bindgen_helper::generate(header_files);
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
