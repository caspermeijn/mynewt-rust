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

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn get_mynewt_core_path() -> Result<PathBuf, String> {
    match env::var("CORE_PATH") {
        Ok(mynewt_core_path) => Ok(PathBuf::from(mynewt_core_path)),
        Err(_) => Err(String::from(
            "Environment variable CORE_PATH should be set to a copy of apache-mynewt-core",
        )),
    }
}

pub fn generate(header_files: Vec<&str>) -> Result<(), String> {
    let mynewt_core_path = get_mynewt_core_path()?;
    let header_paths: Vec<PathBuf> = header_files
        .iter()
        .map(|header_file| mynewt_core_path.join(header_file))
        .collect();

    generate_paths(header_paths)
}

pub fn generate_wrapper() -> Result<(), String> {
    let header_paths = vec![PathBuf::from("wrapper.h")];

    generate_paths(header_paths)
}

pub fn generate_paths(header_paths: Vec<PathBuf>) -> Result<(), String> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    let mut builder = bindgen::Builder::default()
        .clang_arg("--target=thumbv7m-none-eabi")
        .use_core()
        .derive_default(true)
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    for header_path in header_paths {
        builder = builder.header(header_path.to_str().unwrap());
        println!("cargo:rerun-if-changed={}", header_path.to_str().unwrap());
    }
    // If available, set the include directories as mynewt would do
    if let Ok(include_path_list) = env::var("MYNEWT_INCLUDE_PATH") {
        for include_path in include_path_list.split(":") {
            builder = builder.clang_arg("--include-directory=".to_owned() + include_path);
        }
    }
    // If available, set the CFLAGS as mynewt would do
    if let Ok(cflag_list) = env::var("MYNEWT_CFLAGS") {
        for cflag in cflag_list.split(" ") {
            builder = builder.clang_arg(cflag);
        }
    }

    builder
        .generate()
        .map_err(|_| "Failed to generate")?
        .write_to_file(out_path)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    Ok(())
}
