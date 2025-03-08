use crate::SampleSchema;
use rinf::RustSignalBinary;
use serde::Serialize;

/// You can add your custom comments like this.
#[derive(Serialize, RustSignalBinary)]
pub struct SampleFractal {
    pub current_scale: f64,
    pub dummy: Option<SampleSchema>,
}
