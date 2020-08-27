use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum EchoFilterAttr {
    Wet = 0,
    Delay,
    Decay,
    Filter,
}

#[derive(FilterExt)]
pub struct EchoFilter {
    _inner: *mut soloud_sys::soloud::EchoFilter,
}

impl EchoFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = soloud_sys::soloud::EchoFilter_setParams(self._inner, delay);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Set filter params with extra args
    pub fn set_params_ex(
        &mut self,
        delay: f32,
        decay: f32,
        filter: f32,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = soloud_sys::soloud::EchoFilter_setParamsEx(self._inner, delay, decay, filter);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
