use super::ParamType;
use crate::prelude::*;

#[derive(FilterExt)]
pub struct FreeverbFilter {
    _inner: *mut soloud_sys::soloud::FreeverbFilter,
}

impl FreeverbFilter {
    /// Set filter params
    pub fn set_params(
        &mut self,
        mode: f32,
        room_size: f32,
        damp: f32,
        width: f32,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = soloud_sys::soloud::FreeverbFilter_setParams(
                self._inner,
                mode,
                room_size,
                damp,
                width,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
