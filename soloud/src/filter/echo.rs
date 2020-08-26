use crate::prelude::*;
use super::ParamType;

#[repr(u32)]
#[derive(FilterType, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum EchoFilterType {
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
    pub fn set_params(&mut self, aDelay: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::EchoFilter_setParams(self._inner, aDelay);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }


    pub fn set_params_ex( &mut self, aDelay: f32, aDecay: f32, aFilter: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::EchoFilter_setParamsEx(self._inner, aDelay, aDecay, aFilter);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
