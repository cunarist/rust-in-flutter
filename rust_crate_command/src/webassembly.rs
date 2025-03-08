use crate::run_subprocess;
use std::path::Path;
use std::process::Command;

pub fn build_webassembly(
    root_dir: &Path,
    is_release_mode: bool,
    is_internet_connected: bool,
) {
    let total_steps = 3;
    let mut step = 0;

    if is_internet_connected {
        step += 1;
        println!(
            "[{}/{}] Installing Rust toolchain for the web",
            step, total_steps
        );
        install_wasm_toolchain();
    } else {
        println!("Skipping ensurement of Rust toolchain for the web");
    }

    step += 1;
    println!(
        "[{}/{}] Preparing WebAssembly output path",
        step, total_steps
    );
    let output_path = root_dir.join("web").join("pkg");

    step += 1;
    println!(
        "[{}/{}] Compiling Rust with `wasm-pack` to `web` target",
        step, total_steps
    );
    compile_wasm(output_path, is_release_mode);

    println!(
        "[{}/{}] WebAssembly module is now ready 🎉",
        step, total_steps
    );
    println!("To get the Flutter web server command, run `rinf server`");
}

fn install_wasm_toolchain() {
    run_subprocess("rustup", &["toolchain", "install", "nightly"]);
    run_subprocess("rustup", &["+nightly", "component", "add", "rust-src"]);
    run_subprocess(
        "rustup",
        &["+nightly", "target", "add", "wasm32-unknown-unknown"],
    );
    run_subprocess("rustup", &["target", "add", "wasm32-unknown-unknown"]);
    run_subprocess("cargo", &["install", "wasm-pack"]);
    run_subprocess("cargo", &["install", "wasm-bindgen-cli"]);
}

fn compile_wasm(output_path: std::path::PathBuf, is_release_mode: bool) {
    let mut wasm_pack_args = vec![
        "--quiet",
        "build",
        "./native/hub",
        "--out-dir",
        output_path.to_str().unwrap(),
        "--out-name",
        "hub",
        "--no-typescript",
        "--target",
        "web",
        "--",
        "-Z",
        "build-std=std,panic_abort",
    ];
    if !is_release_mode {
        wasm_pack_args.insert(7, "--dev");
    }

    let status = Command::new("wasm-pack")
        .args(&wasm_pack_args)
        .env("RUSTUP_TOOLCHAIN", "nightly")
        .env(
            "RUSTFLAGS",
            "-C target-feature=+atomics,+bulk-memory,+mutable-globals",
        )
        .status()
        .expect("Failed to compile with wasm-pack");

    if !status.success() {
        panic!("Wasm compilation failed");
    }

    println!("Saved `.wasm` and `.js` files to `web/pkg/`");
}
