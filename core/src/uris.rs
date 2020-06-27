//! Commonly used URIs from the lv2plug.in domain associated with lv2-core
//!
use urid::*;

pub struct SampleRate;

unsafe impl UriBound for SampleRate {
    const URI: &'static [u8] = sys::LV2_CORE__sampleRate;
}
