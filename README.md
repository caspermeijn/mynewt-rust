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

This repo contains some bindings for the mynewt api. It seperates the code per
mynewt package. 

Status
------
Incomplete. I will add new APIs when I need them. Feel free to send a pull 
request.

It could be that the bindgen code doesn't build, as the include directories and 
CFLAGS need some changes to newt builder that are not yet upstream.

Usage
-----
You need a BSP
crate which provides the hardware mapping. This means that the pin definition
in bsp.h and the initialisation in hal_bsp.c need to be converted to a struct.
See `mynewt-pinetime-bsp` as an example.

Then you can just add the bsp crate as an dependency of your rust application.
