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

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str;

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

    // If available, set the sysroot as mynewt would do
    if let Ok(cc_path) = env::var("MYNEWT_CC_PATH") {
        let cc_output = Command::new(cc_path)
            .arg("-print-sysroot")
            .output()
            .expect("failed to execute gcc");
        assert!(cc_output.status.success());
        let sysroot_path = str::from_utf8(&cc_output.stdout).unwrap().trim();
        builder = builder.clang_arg(format!("--sysroot={}", sysroot_path));
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

fn main() {
    println!("cargo:rerun-if-env-changed=MYNEWT_PACKAGES");
    let package_names = env::var("MYNEWT_PACKAGES").unwrap();
    let package_names = package_names.split(":");

    let header_files = package_names
        .map(|name| {PathBuf::new().join("src").join(name).join("wrapper.h")})
        .filter(|path| { path.exists() })
        .collect();

    let result = generate_paths(header_files);
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
