<!--
SPDX-License-Identifier: Apache-2.0
Copyright 2020 Casper Meijn <casper@meijn.net>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
-->

Rust bindings for mynewt
========================

This repo contains some rust bindings for the mynewt api. It contains two crates:

- mynewt-sys contains the raw rust bindings
- mynewt contains safe wrappers for the mynewt packages

Both crates only build the bindings for the enabled mynewt packages. 

Status
------
Incomplete. I will add new APIs when I need them. Feel free to send a pull 
request.

The bindgen code expects some environment variables that are provided by the 
master branch of newt builder. See https://github.com/caspermeijn/mynewt-newt/blob/master/INSTALLING.md

Usage
-----
First you need to add the mynewt crate as an dependency of your rust application. 
Then you can use the correct BSP module for obtaining the peripherals.
