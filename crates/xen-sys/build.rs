use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(
        std::env::var("OUT_DIR").expect("Unable to get OUT_DIR environment variable"),
    );

    let mut args = Vec::new();
    let config = pkg_config::Config::new();

    if std::env::var("DOCS_RS").is_ok() || std::env::var("XEN_SYS_USE_BINDINGS").is_ok() {
        let src = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let dst = PathBuf::from(std::env::var("OUT_DIR").unwrap());

        let bindings = cfg_select! {
            feature = "bindings-4_20" => "bindings/xen-4.20.rs",
            feature = "bindings-4_21" => "bindings/xen-4.21.rs",
            feature = "bindings-4_22" => "bindings/xen-4.22.rs",
            _ => "bindings/xen-4.22.rs",
        };

        std::fs::copy(src.join(bindings), dst.join("bindings.rs"))
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
        // wrap_unsafe_ops might be enabled by default in the future
        // when the target edition is set to 2024
        // https://github.com/rust-lang/rust-bindgen/issues/3147
        .wrap_unsafe_ops(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
