use std::{env, fs, path::PathBuf};

fn main() {
    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("Unable to get OUT_DIR environment variable"));

    let mut args = Vec::new();
    let config = pkg_config::Config::new();

    if env::var("DOCS_RS").is_ok()
        || env::var("XEN_SYS_USE_BINDINGS").is_ok()
       // || cfg!(feature = "bindings-4_20")
    {
        let src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dst = PathBuf::from(env::var("OUT_DIR").unwrap());

        fs::copy(src.join("bindings/xen-4.20.rs"), dst.join("bindings.rs"))
            .expect("Failed to copy bindings.rs");

        return;
    }

    if cfg!(feature = "xencontrol") {
        config
            .probe("xencontrol")
            .expect("Failed to locate xencontrol library");
        args.extend(["-D", "BINDGEN_XENCONTROL"]);
    }

    if cfg!(feature = "xendevicemodel") {
        config
            .probe("xendevicemodel")
            .expect("Failed to locate xendevicemodel library");
        args.extend(["-D", "BINDGEN_XENDEVICEMODEL"]);
    }

    if cfg!(feature = "xenevtchn") {
        config
            .probe("xenevtchn")
            .expect("Failed to locate xenevtchn library");
        args.extend(["-D", "BINDGEN_XENEVTCHN"]);
    }

    if cfg!(feature = "xenforeignmemory") {
        config
            .probe("xenforeignmemory")
            .expect("Failed to locate xenforeignmemory library");
        args.extend(["-D", "BINDGEN_XENFOREIGNMEMORY"]);
    }

    if cfg!(feature = "xenstore") {
        config
            .probe("xenstore")
            .expect("Failed to locate xenstore library");
        args.extend(["-D", "BINDGEN_XENSTORE"]);
    }

    if cfg!(feature = "vm_event") {
        args.extend(["-D", "BINDGEN_VM_EVENT"]);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(args)
        .derive_debug(true)
        .derive_default(true)
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
