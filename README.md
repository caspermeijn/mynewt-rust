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
