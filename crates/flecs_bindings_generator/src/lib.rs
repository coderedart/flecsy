//! Bindings generator for flecs
//!
//! This crate does multiple things:
//! 1. generate bindings for native flecs. This is useful if you are just using it natively with rust
//! 2. generate bindings for wasm flecs. This is to "separate" bindings for wasm and native flecs
//! 3. generate bindings for wasm from hostside using wasmtime has the reference.
//!
mod generator;

use anyhow::{Context, Result};
use bindgen::Builder;

pub fn generate_bindings() -> Result<()> {
    let bindings = Builder::default()
        .header("./flecs.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate_cstr(true)
        .merge_extern_blocks(true)
        .layout_tests(false)
        .array_pointers_in_arguments(true)
        .wrap_unsafe_ops(true)
        .prepend_enum_name(false)
        .raw_line(
            r#"
        #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
        "#,
        )
        // .parse_callbacks(Box::new(FlecsParseCB::default()))
        .generate()
        .expect("Unable to generate bindings")
        .to_string();
    generator::generate_safe_wrappers(&bindings).context("failed to generate safe wrappers")?;
    std::fs::write("./crates/flecsys/src/lib.rs", bindings)
        .context("failed to write raw bindings.flecsys/lib.rs")?;
    Ok(())
}
pub enum BindingsPlatform {
    Native,
    Wasm,
    WasmHost,
    LuaHost,
}
