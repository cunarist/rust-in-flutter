#[macro_export]
/// Writes the interface code
/// needed to communicate with Dart.
/// This should be used once, and only once,
/// at the root of the `hub` crate.
macro_rules! write_interface {
    () => {
        #[cfg(not(target_family = "wasm"))]
        #[no_mangle]
        pub extern "C" fn rinf_start_rust_logic_extern() {
            let result = $crate::start_rust_logic(main);
            if let Err(error) = result {
                rinf::debug_print!("{error}");
            }
        }

        #[cfg(target_family = "wasm")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn rinf_start_rust_logic_extern() {
            let result = $crate::start_rust_logic(main);
            if let Err(error) = result {
                rinf::debug_print!("{error}");
            }
        }
    };
}

/// Delegates the printing operation to Flutter,
/// which excels at handling various platforms
/// including web and mobile emulators.
/// When debugging, using this macro is recommended over `println!()`,
/// as it seamlessly adapts to different environments.
/// Note that this macro does nothing in release mode.
#[macro_export]
macro_rules! debug_print {
    ( $( $t:tt )* ) => {
        let rust_report = format!( $( $t )* );
        #[cfg(debug_assertions)]
        {
            let result = $crate::send_rust_signal(
                "RinfPrint", // This is a special message ID for Rust reports
                Vec::new(),
                rust_report.clone().into_bytes(),
            );
            if let Err(error) = result {
                println!("{error}\n{rust_report}");
            }
        }
        #[cfg(not(debug_assertions))]
        let _ = rust_report;
    }
}
