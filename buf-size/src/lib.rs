extern crate lv2_sys as sys;

use urid::*;

mod buf_size {
    use urid::UriBound;

    pub struct BoundedBlockLength;

    unsafe impl UriBound for BoundedBlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__boundedBlockLength;
    }

    pub struct FixedBlockLength;

    unsafe impl UriBound for FixedBlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__fixedBlockLength;
    }

    pub struct MaxBlockLength;

    unsafe impl UriBound for MaxBlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__maxBlockLength;
    }

    pub struct MinBlockLength;

    unsafe impl UriBound for MinBlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__minBlockLength;
    }

    pub struct NominalBlockLength;

    unsafe impl UriBound for NominalBlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__nominalBlockLength;
    }

    pub struct PowerOf2BlockLength;

    unsafe impl UriBound for PowerOf2BlockLength {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__powerOf2BlockLength;
    }

    pub struct SequenceSize;

    unsafe impl UriBound for SequenceSize {
        const URI: &'static [u8] = sys::LV2_BUF_SIZE__maxBlockLength;
    }
}

use buf_size::*;

/// A URID cache containing buf size properties
#[derive(URIDCollection)]
pub struct BufSizeURIDCollection {
    pub bounded_block_length: URID<BoundedBlockLength>,
    pub fixed_block_length: URID<FixedBlockLength>,
    pub max_block_length: URID<MaxBlockLength>,
    pub min_block_length: URID<MinBlockLength>,
    pub nominal_block_length: URID<NominalBlockLength>,
    pub power_of_2_block_length: URID<PowerOf2BlockLength>,
    pub sequence_size: URID<SequenceSize>,
}

/// Prelude of `lv2_buf-size` for wildcard usage.
pub mod prelude {
    pub use crate::buf_size::*;
    pub use crate::BufSizeURIDCollection;
}
