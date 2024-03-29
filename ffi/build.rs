extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let rte_sdk = env::var("RTE_SDK").expect("No RTE_SDK found ~ DPDK installation directory.");

    // there's a problem statically linking to a linker script
    // see: https://github.com/rust-lang/rust/issues/40483
    println!("cargo:rustc-link-search=native={}/build/lib", rte_sdk);
    println!("cargo:rustc-link-lib=dylib=dpdk");
    println!("cargo:rustc-link-lib=dylib=numa");
    println!("cargo:rustc-link-lib=dylib=pcap");
    println!("cargo:rustc-link-lib=dylib=z");

    cc::Build::new()
        .file("src/shim.c")
        .include("/opt/dpdk/build/include")
        .flag("-march=native")
        .compile("rte_shim");

    let bindings = bindgen::Builder::default()
        .header("src/rte.h")
        .header("src/shim.h")
        .generate_comments(true)
        .generate_inline_functions(true)
        .whitelist_type(r"(rte|cmdline|ether|eth|arp|vlan|vxlan)_.*")
        .whitelist_function(r"(_rte|rte|cmdline|lcore|ether|eth|arp|is)_.*")
        .whitelist_var(
            r"(RTE|CMDLINE|ETHER|ARP|VXLAN|BONDING|LCORE|MEMPOOL|ARP|PKT|EXT_ATTACHED|IND_ATTACHED|lcore|rte|cmdline|per_lcore)_.*",
        )
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg("-fkeep-inline-functions")
        .clang_arg("-I/opt/dpdk/build/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
