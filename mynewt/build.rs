use std::env;

pub fn emit_mynewt_packages_as_cfg() {
    println!("cargo:rerun-if-env-changed=MYNEWT_PACKAGES");
    let packages = env::var("MYNEWT_PACKAGES").unwrap();
    for name in packages.split(":") {
        println!("cargo:rustc-cfg=mynewt_package=\"{}\"", name)
    }
}

fn main() {
    emit_mynewt_packages_as_cfg();
}
