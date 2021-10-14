use super::ParamType;
use crate::prelude::*;

/// Freeverb filter
#[derive(Debug)]
pub struct FreeverbFilter {
    inner: *mut soloud_sys::soloud::FreeverbFilter,
}

crate::macros::filter::impl_filter_ext!(FreeverbFilter);

impl FreeverbFilter {
    /// Set filter params
    pub fn set_params(
        &mut self,
        mode: f32,
        room_size: f32,
        damp: f32,
        width: f32,
    ) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::FreeverbFilter_setParams(
            self.inner, mode, room_size, damp, width,
        ))
    }
}
