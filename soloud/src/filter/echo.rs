use crate::prelude::*;
use super::ParamType;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum EchoFilterAttr {
    Wet = 0,
    Delay,
    Decay,
    Filter
}

#[derive(FilterExt)]
pub struct EchoFilter {
    _inner: *mut soloud_sys::soloud::EchoFilter,
}

impl EchoFilter {
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::EchoFilter_setParams(self._inner, delay);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }


    pub fn set_params_ex(&mut self, delay: f32, decay: f32, filter: f32) -> Result<(), SoloudError> {
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
