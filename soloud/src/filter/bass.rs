use super::ParamType;
use crate::prelude::*;

/// Bass boost filter attribute
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BassBoostFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Boost attribute
    Boost = 1,
}

/// Bass boost filter
#[derive(Debug)]
pub struct BassboostFilter {
    inner: *mut soloud_sys::soloud::BassboostFilter,
}

crate::macros::filter::impl_filter_ext!(BassboostFilter);
crate::macros::filter::impl_filter_type!(BassBoostFilterAttr);

impl BassboostFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::BassboostFilter_setParams(
            self.inner, delay
        ))
    }
}
