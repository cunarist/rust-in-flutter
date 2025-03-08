use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// TODO: Prohibit `Os`/`Web` suffix.

/// Marks the struct as a signal
/// that can be nested within other signals.
/// A `SignalPiece` cannot be sent independently
/// and is only a partial component of `DartSignal` or `RustSignal`.
#[proc_macro_derive(SignalPiece)]
pub fn derive_signal_piece(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// TODO: Hide generated code from intellisense autocomplete.

/// Marks the struct as a signal endpoint
/// that contains a message from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignal)]
pub fn derive_dart_signal(input: TokenStream) -> TokenStream {
    derive_dart_signal_real(input)
}

/// Marks the struct as a signal endpoint
/// that contains a message and binary from Dart to Rust.
/// This can be marked on any type that implements `Deserialize`.
#[proc_macro_derive(DartSignalBinary)]
pub fn derive_dart_signal_binary(input: TokenStream) -> TokenStream {
    derive_dart_signal_real(input)
}

fn derive_dart_signal_real(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let name_lit = name.to_string();
    let snake_name = name_lit.to_case(Case::Snake);
    let upper_snake_name = name_lit.to_case(Case::UpperSnake);

    let channel_type_ident =
        Ident::new(&format!("{}Channel", name), name.span());
    let channel_const_ident =
        Ident::new(&format!("{}_CHANNEL", upper_snake_name), name.span());
    let extern_fn_ident = Ident::new(
        &format!("rinf_send_dart_signal_{}", snake_name),
        name.span(),
    );

    let expanded = quote! {
        impl #name {
            pub fn get_dart_signal_receiver(
            ) -> rinf::SignalReceiver<rinf::DartSignal<Self>> {
                #channel_const_ident.1.clone()
            }

            fn send_dart_signal(message_bytes: &[u8], binary: &[u8]){
                use bincode::deserialize;
                use rinf::{debug_print, RinfError};
                let message_result: Result<#name, RinfError> =
                    deserialize(message_bytes)
                    .map_err(|_| RinfError::CannotDecodeMessage);
                let message = match message_result {
                    Ok(inner) => inner,
                    Err(error) => {
                        let type_name = #name_lit;
                        debug_print!("{}: \n{}", type_name, error);
                        return;
                    }
                };
                let dart_signal = DartSignal {
                    message,
                    binary: binary.to_vec(),
                };
                #channel_const_ident.0.send(dart_signal);
            }
        }

        type #channel_type_ident = std::sync::LazyLock<(
            rinf::SignalSender<rinf::DartSignal<#name>>,
            rinf::SignalReceiver<rinf::DartSignal<#name>>,
        )>;

        static #channel_const_ident: #channel_type_ident =
            std::sync::LazyLock::new(rinf::signal_channel);

        #[cfg(not(target_family = "wasm"))]
        #[no_mangle]
        pub unsafe extern "C" fn #extern_fn_ident(
            message_pointer: *const u8,
            message_size: usize,
            binary_pointer: *const u8,
            binary_size: usize,
        ) {
            use std::slice::from_raw_parts;
            let message_bytes = from_raw_parts(message_pointer, message_size);
            let binary = from_raw_parts(binary_pointer, binary_size);
            #name::send_dart_signal(message_bytes, binary);
        }

        #[cfg(target_family = "wasm")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn #extern_fn_ident(
            message_bytes: &[u8],
            binary: &[u8],
        ) {
            #name::send_dart_signal(message_bytes, binary);
        }
    };

    TokenStream::from(expanded)
}

/// Marks the struct as a signal endpoint
/// that contains a message from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignal)]
pub fn derive_rust_signal(input: TokenStream) -> TokenStream {
    derive_rust_signal_real(input, false)
}

/// Marks the struct as a signal endpoint
/// that contains a message and binary from Rust to Dart.
/// This can be marked on any type that implements `Serialize`.
#[proc_macro_derive(RustSignalBinary)]
pub fn derive_rust_signal_binary(input: TokenStream) -> TokenStream {
    derive_rust_signal_real(input, true)
}

fn derive_rust_signal_real(
    input: TokenStream,
    include_binary: bool,
) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let name_lit = name.to_string();

    let expanded = if include_binary {
        quote! {
            impl #name {
                pub fn send_signal_to_dart(self, binary: Vec<u8>) {
                    use bincode::serialize;
                    use rinf::{debug_print, send_rust_signal, RinfError};
                    let type_name = #name_lit;
                    let message_result: Result<Vec<u8>, RinfError> =
                        serialize(&self)
                        .map_err(|_| RinfError::CannotEncodeMessage);
                    let message_bytes = match message_result {
                        Ok(inner) => inner,
                        Err(error) => {
                            debug_print!("{}: \n{}", type_name, error);
                            return;
                        }
                    };
                    let result =
                        send_rust_signal(type_name, message_bytes, binary);
                    if let Err(error) = result {
                        debug_print!("{}: \n{}", type_name, error);
                    }
                }
            }
        }
    } else {
        quote! {
            impl #name {
                pub fn send_signal_to_dart(self) {
                    use bincode::serialize;
                    use rinf::{debug_print, send_rust_signal, RinfError};
                    let type_name = #name_lit;
                    let message_result: Result<Vec<u8>, RinfError> =
                        serialize(&self)
                        .map_err(|_| RinfError::CannotEncodeMessage);
                    let message_bytes = match message_result {
                        Ok(inner) => inner,
                        Err(error) => {
                            debug_print!("{}: \n{}", type_name, error);
                            return;
                        }
                    };
                    let result =
                        send_rust_signal(type_name, message_bytes, Vec::new());
                    if let Err(error) = result {
                        debug_print!("{}: \n{}", type_name, error);
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// TODO: Enforce trait-based type safety
