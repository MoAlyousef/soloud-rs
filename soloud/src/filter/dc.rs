use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct DCRemovalFilter {
    _inner: *mut soloud_sys::soloud::DCRemovalFilter,
}

impl DCRemovalFilter {
    pub fn set_params(&mut self) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::DCRemovalFilter_setParams(self._inner);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn set_params_ex(&mut self, delay: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::DCRemovalFilter_setParamsEx(self._inner, delay);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}