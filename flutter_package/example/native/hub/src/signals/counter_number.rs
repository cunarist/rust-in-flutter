use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct SampleNumberInput {
    pub letter: String,
    pub dummy_one: u32,
    pub dummy_two: Option<SampleSchema>,
    pub dummy_three: Vec<i32>,
}

#[derive(Serialize, RustSignal)]
pub struct SampleNumberOutput {
    pub current_number: i32,
    pub dummy_one: u32,
    pub dummy_two: Option<SampleSchema>,
    pub dummy_three: Vec<i32>,
}

#[derive(Serialize, Deserialize, SignalPiece)]
pub struct SampleSchema {
    pub sample_field_one: bool,
    pub sample_field_two: bool,
}
