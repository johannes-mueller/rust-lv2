#[macro_use]
extern crate bitflags;

extern crate lv2_atom as atom;
extern crate lv2_core as core;
extern crate lv2_sys as sys;
extern crate urid;

pub mod feature;
pub mod interface;

pub use feature::LV2Options;
pub use interface::*;

bitflags! {
    pub struct OptionsErr: u32 {
        const UNKNOWN = sys::LV2_Options_Status_LV2_OPTIONS_ERR_UNKNOWN;
        const BAD_SUBJECT = sys::LV2_Options_Status_LV2_OPTIONS_ERR_BAD_SUBJECT;
        const BAD_KEY = sys::LV2_Options_Status_LV2_OPTIONS_ERR_BAD_KEY;
        const BAD_VALUE = sys::LV2_Options_Status_LV2_OPTIONS_ERR_BAD_VALUE;
    }
}

#[derive(Debug)]
pub enum Subject {
    Instance,
    Resource(urid::URID),
    Blank(u32),
    Port(u32),
}
