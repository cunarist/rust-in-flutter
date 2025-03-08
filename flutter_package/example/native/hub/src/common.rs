// `tokio_with_wasm` enables `tokio` code
// to run directly on the web.
pub use tokio_with_wasm::alias as tokio;

/// This `Result` type alias unifies the error type.
/// Building an app differs from writing a library,
/// as app may encounter numerous error situations.
/// Therefore, a single, flexible error type is recommended.
pub type Result<T> = anyhow::Result<T>;

// TODO: Always import directly from other crates

/// Because spawn functions are used very often,
/// we make them accessible from everywhere.
pub use tokio::task::{spawn, spawn_blocking};
