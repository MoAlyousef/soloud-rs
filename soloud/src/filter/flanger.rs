use super::ParamType;
use crate::prelude::*;

/// Flanger filter attributes
#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FlangerFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Delay attribute
    Delay = 1,
    /// Frequency attribute
    Freq = 2,
}

/// Flanger filter
#[derive(Debug, FilterExt)]
pub struct FlangerFilter {
    _inner: *mut soloud_sys::soloud::FlangerFilter,
}

impl FlangerFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32, freq: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::FlangerFilter_setParams(
            self._inner,
            delay,
            freq
        ))
    }
}
