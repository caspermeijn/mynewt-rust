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
        "hw/bus/include/bus/bus.h",
        "hw/bus/drivers/spi_common/include/bus/drivers/spi_common.h",
    ];

    let result = mynewt_bindgen_helper::generate(header_files);
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}