use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LofiFilterAttr {
    Wet = 0,
    Samplerate = 1,
    BitDepth = 2,
}

#[derive(FilterExt)]
pub struct LofiFilter {
    _inner: *mut soloud_sys::soloud::LofiFilter,
}

impl LofiFilter {
    pub fn set_params(&mut self, samplerate: f32, bit_depth: f32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = soloud_sys::soloud::LofiFilter_setParams(self._inner, samplerate, bit_depth);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
