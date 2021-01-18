use super::ParamType;
use crate::prelude::*;

/// Lofi filter attributes
#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LofiFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Samplerate attribute
    Samplerate = 1,
    /// Bitdepth attribute
    BitDepth = 2,
}

/// Lofi filter
#[derive(Debug, FilterExt)]
pub struct LofiFilter {
    _inner: *mut soloud_sys::soloud::LofiFilter,
}

impl LofiFilter {
    /// Set filter params
    pub fn set_params(&mut self, samplerate: f32, bit_depth: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::LofiFilter_setParams(
            self._inner,
            samplerate,
            bit_depth
        ))
    }
}
