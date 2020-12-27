use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum WaveShaperFilterAttr {
    Wet = 0,
    Amount = 1,
}

#[derive(Debug, FilterExt)]
pub struct WaveShaperFilter {
    _inner: *mut soloud_sys::soloud::WaveShaperFilter,
}

impl WaveShaperFilter {
    /// Set filter params
    pub fn set_params(&mut self, amount: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::WaveShaperFilter_setParams(self._inner, amount);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
