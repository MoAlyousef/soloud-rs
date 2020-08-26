use crate::prelude::*;
use super::ParamType;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LofiFilterAttr {
    Wet = 0,
    SampleRate = 1,
    BitDepth = 2,
}

#[derive(FilterExt)]
pub struct LofiFilter {
    _inner: *mut soloud_sys::soloud::LofiFilter,
}

impl LofiFilter {
    pub fn set_params(&mut self, aSampleRate: f32, aBitDepth: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::LofiFilter_setParams(self._inner, aSampleRate, aBitDepth);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}