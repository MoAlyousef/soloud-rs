use super::ParamType;
use crate::prelude::*;

/// Wave shaper filter attributes
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum WaveShaperFilterAttr {
    /// "Wet" attribute
    Wet = 0,
    /// "Amount" attribute
    Amount = 1,
}

/// Wrapper around the wave shaper filter
#[derive(Debug)]
pub struct WaveShaperFilter {
    inner: *mut soloud_sys::soloud::WaveShaperFilter,
}

crate::macros::filter::impl_filter_ext!(WaveShaperFilter);
crate::macros::filter::impl_filter_type!(WaveShaperFilterAttr);

impl WaveShaperFilter {
    /// Set filter params
    pub fn set_params(&mut self, amount: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::WaveShaperFilter_setParams(
            self.inner, amount
        ))
    }
}
