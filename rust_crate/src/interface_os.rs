use crate::error::RinfError;
use crate::shutdown::SHUTDOWN_EVENTS;
use allo_isolate::ffi::DartPostCObjectFnType;
use allo_isolate::{
    store_dart_post_cobject, IntoDart, Isolate, ZeroCopyBuffer,
};
use os_thread_local::ThreadLocal;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::thread;

static DART_ISOLATE: Mutex<Option<Isolate>> = Mutex::new(None);

#[no_mangle]
pub unsafe extern "C" fn rinf_prepare_isolate_extern(
    store_post_object: DartPostCObjectFnType,
    port: i64,
) {
    store_dart_post_cobject(store_post_object);
    let dart_isolate = Isolate::new(port);
    let mut guard = match DART_ISOLATE.lock() {
        Ok(inner) => inner,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.replace(dart_isolate);
}

// We use `os_thread_local` so that when the program fails
// and the main thread exits unexpectedly,
// the whole Rust async runtime shuts down accordingly.
// Without this solution,
// zombie threads inside the Rust async runtime might outlive the app.
// This `ThreadLocal` is intended to be used only on the main thread,
type ShutdownDropperLock = OnceLock<ThreadLocal<ShutdownDropper>>;
static SHUTDOWN_DROPPER: ShutdownDropperLock = OnceLock::new();

/// Notifies Rust that Dart thread has exited when dropped.
pub struct ShutdownDropper;

impl Drop for ShutdownDropper {
    fn drop(&mut self) {
        SHUTDOWN_EVENTS.dart_stopped.set();
        SHUTDOWN_EVENTS.rust_stopped.wait();
    }
}

pub fn start_rust_logic_real<F, T>(main_fn: F) -> Result<(), RinfError>
where
    F: Fn() -> T + Send + 'static,
{
    // Enable console output for panics.
    #[cfg(debug_assertions)]
    {
        #[cfg(not(feature = "backtrace"))]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                crate::debug_print!("A panic occurred in Rust.\n{panic_info}");
            }));
        }
        #[cfg(feature = "backtrace")]
        {
            std::panic::set_hook(Box::new(|panic_info| {
                let backtrace = backtrace::Backtrace::new();
                crate::debug_print!(
                    "A panic occurred in Rust.\n{panic_info}\n{backtrace:?}"
                );
            }));
        }
    }

    // Prepare the shutdown dropper that will notify the Rust async runtime
    // after Dart thread has exited.
    // This code assumes that this is the main thread.
    let thread_local = ThreadLocal::new(|| ShutdownDropper);
    let _ = SHUTDOWN_DROPPER.set(thread_local);

    // Spawn a new thread to run the async runtime.
    thread::spawn(move || {
        // Notify that Dart has stopped
        // to terminate the previous Rust async runtime threads.
        // After Dart's hot restart or reopening the app,
        // Previous Rust async runtime can be still running.
        SHUTDOWN_EVENTS.dart_stopped.set();

        // Clear shutdown events to prepare for a fresh start.
        SHUTDOWN_EVENTS.dart_stopped.clear();
        SHUTDOWN_EVENTS.rust_stopped.clear();

        // Execute the long-running function that will block the thread
        // for the entire lifecycle of the app.
        // This function runs the async Rust runtime.
        main_fn();

        // After the Rust async runtime is closed,
        // notify the main Dart thread to stop blocking
        // and allow the application to exit.
        SHUTDOWN_EVENTS.rust_stopped.set();
    });

    Ok(())
}

#[no_mangle]
pub extern "C" fn rinf_stop_rust_logic_extern() {
    SHUTDOWN_EVENTS.dart_stopped.set();
}

pub fn send_rust_signal_real(
    endpoint: &str,
    message_bytes: Vec<u8>,
    binary: Vec<u8>,
) -> Result<(), RinfError> {
    // When `DART_ISOLATE` is not initialized, just return the error.
    // This can happen when running test code in Rust.
    let guard = match DART_ISOLATE.lock() {
        Ok(inner) => inner,
        Err(poisoned) => poisoned.into_inner(),
    };
    let dart_isolate = guard.as_ref().ok_or(RinfError::NoDartIsolate)?;

    // If a `Vec<u8>` is empty, we can't just simply send it to Dart
    // because panic can occur from null pointers.
    // Instead, we will reconstruct the empty vector from the Dart side.
    let message_filled = !message_bytes.is_empty();
    let binary_filled = !binary.is_empty();

    dart_isolate.post(
        vec![
            endpoint.into_dart(),
            if message_filled {
                ZeroCopyBuffer(message_bytes).into_dart()
            } else {
                ().into_dart()
            },
            if binary_filled {
                ZeroCopyBuffer(binary).into_dart()
            } else {
                ().into_dart()
            },
        ]
        .into_dart(),
    );

    Ok(())
}
