use super::ParamType;
use crate::prelude::*;

/// Lofi filter attributes
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LofiFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Samplerate attribute
    Samplerate = 1,
    /// Bitdepth attribute
    BitDepth = 2,
}

/// Lofi filter
#[derive(Debug)]
pub struct LofiFilter {
    inner: *mut soloud_sys::soloud::LofiFilter,
}

crate::macros::filter::impl_filter_ext!(LofiFilter);
crate::macros::filter::impl_filter_type!(LofiFilterAttr);

impl LofiFilter {
    /// Set filter params
    pub fn set_params(&mut self, samplerate: f32, bit_depth: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::LofiFilter_setParams(
            self.inner, samplerate, bit_depth
        ))
    }
}
