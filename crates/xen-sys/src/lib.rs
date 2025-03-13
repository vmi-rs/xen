#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::useless_transmute,

    // remove when a new version of bindgen is released
    // https://github.com/rust-lang/rust-bindgen/pull/3124
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    clippy::ptr_offset_with_cast,
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
