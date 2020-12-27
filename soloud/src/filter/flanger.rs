use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FlangerFilterAttr {
    Wet = 0,
    Delay = 1,
    Freq = 2,
}

#[derive(Debug, FilterExt)]
pub struct FlangerFilter {
    _inner: *mut soloud_sys::soloud::FlangerFilter,
}

impl FlangerFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32, freq: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::FlangerFilter_setParams(self._inner, delay, freq);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
