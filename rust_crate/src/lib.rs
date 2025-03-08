mod channel;
mod error;
mod macros;
mod shutdown;

mod interface;
#[cfg(not(target_family = "wasm"))]
mod interface_os;
#[cfg(target_family = "wasm")]
mod interface_web;

pub use channel::{signal_channel, SignalReceiver, SignalSender};
pub use error::RinfError;
pub use interface::{send_rust_signal, start_rust_logic, DartSignal};
pub use shutdown::dart_shutdown;

pub use rinf_proc::{
    DartSignal, DartSignalBinary, RustSignal, RustSignalBinary, SignalPiece,
};
