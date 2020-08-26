use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct FreeverbFilter {
    _inner: *mut soloud_sys::soloud::FreeverbFilter,
}

impl FreeverbFilter {
    pub fn set_params(&mut self, aMode: f32, aRoomSize: f32, aDamp: f32, aWidth: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::FreeverbFilter_setParams(self._inner, aMode, aRoomSize, aDamp, aWidth);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}