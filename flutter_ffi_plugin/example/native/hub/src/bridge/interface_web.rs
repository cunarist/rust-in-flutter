use super::interface::*;
use rinf::externs::js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_rust_logic_extern() {
    start_rust_logic();
}

#[wasm_bindgen]
pub fn stop_rust_logic_extern() {
    stop_rust_logic();
}

#[wasm_bindgen]
pub fn request_to_rust_extern(
    interaction_id: i32,
    resource: i32,
    operation: i32,
    message_raw: &[u8],
    blob_raw: &[u8],
) {
    let message_bytes = message_raw.to_vec();
    let message = if message_bytes.len() == 0 {
        None
    } else {
        Some(message_bytes)
    };

    let blob_bytes = blob_raw.to_vec();
    let blob = if blob_bytes.len() == 0 {
        None
    } else {
        Some(blob_bytes)
    };

    let operation_enum;
    if operation == 0 {
        operation_enum = RustOperation::Create;
    } else if operation == 1 {
        operation_enum = RustOperation::Read;
    } else if operation == 2 {
        operation_enum = RustOperation::Update;
    } else {
        operation_enum = RustOperation::Delete;
    }

    let rust_request = RustRequest {
        resource: resource as i32,
        operation: operation_enum,
        message,
        blob,
    };

    let rust_request_unique = RustRequestUnique {
        id: interaction_id as i32,
        request: rust_request,
    };

    request_to_rust(rust_request_unique);
}

#[wasm_bindgen]
pub fn prepare_isolates_extern(port_signal: i32, port_response: i32, port_report: i32) {
    // This function does nothing on the web.
    let _ = port_signal;
    let _ = port_response;
    let _ = port_report;
}

#[wasm_bindgen]
pub fn prepare_channels_extern() {
    prepare_channels();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = rinf_send_rust_signal_extern)]
    pub fn send_rust_signal_extern(resource: i32, message_raw: Uint8Array, blob_raw: Uint8Array);
    #[wasm_bindgen(js_name = rinf_respond_to_dart_extern)]
    pub fn respond_to_dart_extern(
        id: i32,
        successful: bool,
        message_raw: Uint8Array,
        blob_raw: Uint8Array,
    );
    #[wasm_bindgen(js_name = rinf_send_rust_report_extern)]
    pub fn send_rust_report_extern(rust_report: String);
}
