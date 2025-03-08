use std::net::ToSocketAddrs;
use std::process::{Command, ExitStatus};

pub fn run_dart_command(args: &[&str]) -> std::io::Result<()> {
    #[cfg(target_family = "windows")]
    let cmd = "dart.bat";
    #[cfg(target_family = "unix")]
    let cmd = "dart";
    #[cfg(target_family = "wasm")]
    let cmd = "none"; // Dummy value to suppress unused warnings
    run_subprocess(cmd, args);
    Ok(())
}

pub fn run_subprocess(cmd: &str, args: &[&str]) -> ExitStatus {
    Command::new(cmd)
        .args(args)
        .status()
        .expect("Failed to execute command")
}

pub fn check_internet_connection() -> bool {
    "pub.dev:80"
        .to_socket_addrs()
        .map(|mut addrs| addrs.next().is_some())
        .unwrap_or(false)
}
