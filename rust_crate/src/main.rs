#[cfg(not(target_family = "wasm"))]
fn main() -> Result<(), rinf_command::RinfCommandError> {
    rinf_command::run_command()
    // TODO: Check that `rinf_command` crate is only embedded in the binary.
    // TODO: Think about the return type
}

#[cfg(target_family = "wasm")]
fn main() -> Result<(), String> {
    // Dummy function to make the linter happy.
    Ok(())
}
