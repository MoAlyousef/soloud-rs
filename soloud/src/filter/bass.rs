use super::ParamType;
use crate::prelude::*;

/// Bass boost filter attribute
#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BassBoostFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Boost attribute
    Boost = 1,
}

/// Bass boost filter
#[derive(Debug, FilterExt)]
pub struct BassboostFilter {
    _inner: *mut soloud_sys::soloud::BassboostFilter,
}

impl BassboostFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::BassboostFilter_setParams(
            self._inner,
            delay
        ))
    }
}
