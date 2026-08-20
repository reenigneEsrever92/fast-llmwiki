//! Build the client (WASM hydration) bundle and make it available to the
//! server via `OUT_DIR`, so the SSR server can embed it with `include_bytes!`
//! and ship as a single binary.
//!
//! This runs the client build as a nested `cargo` invocation with a separate
//! target directory (`target/client`) to avoid deadlocking on the build lock
//! held by the outer cargo process. The `wasm32-*` guard prevents recursion:
//! when this script is itself run as part of that nested wasm32 build, it
//! returns immediately.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    if target.starts_with("wasm32") {
        // We are compiling the client itself; do not build it again.
        return;
    }

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("expected fawi-gui to live at <workspace>/crates/fawi-gui")
        .to_path_buf();

    let client_target_dir = workspace_root.join("target").join("client");

    // Build the client in the same profile as the server so a `--release`
    // server build embeds an optimized (rather than debug) client bundle.
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let release = profile == "release";

    let mut cargo = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()));
    cargo
        .current_dir(&workspace_root)
        .env("CARGO_TARGET_DIR", &client_target_dir)
        .args([
            "build",
            "--lib",
            "-p",
            "fawi-gui",
            "--target",
            "wasm32-unknown-unknown",
            "--features",
            "hydrate",
            "--no-default-features",
        ]);
    if release {
        cargo.arg("--release");
    }
    let status = cargo
        .status()
        .expect("failed to run cargo to build the client bundle");
    assert!(
        status.success(),
        "client (wasm32) build failed; is the `wasm32-unknown-unknown` target installed?"
    );

    let wasm = client_target_dir
        .join("wasm32-unknown-unknown")
        .join(&profile)
        .join("fawi_gui.wasm");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    let status = Command::new("wasm-bindgen")
        .arg(&wasm)
        .args([
            "--target",
            "web",
            "--out-dir",
            out_dir.to_str().expect("OUT_DIR not UTF-8"),
            "--out-name",
            "okf",
            "--no-typescript",
        ])
        .status()
        .expect("failed to run wasm-bindgen; is it installed?");
    assert!(
        status.success(),
        "wasm-bindgen failed; ensure the CLI version matches the wasm-bindgen crate \
         version (`cargo install wasm-bindgen-cli`)"
    );

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../fawi-core/src");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
