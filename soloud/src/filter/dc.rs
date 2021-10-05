use super::ParamType;
use crate::prelude::*;

/// DC removal filter
#[derive(Debug)]
pub struct DCRemovalFilter {
    inner: *mut soloud_sys::soloud::DCRemovalFilter,
}

crate::macros::filter::impl_filter_ext!(DCRemovalFilter);

impl DCRemovalFilter {
    /// Set filter params
    pub fn set_params(&mut self) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::DCRemovalFilter_setParams(self.inner))
    }

    /// Set filter params with extra args
    pub fn set_params_ex(&mut self, delay: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::DCRemovalFilter_setParamsEx(
            self.inner,
            delay
        ))
    }
}
