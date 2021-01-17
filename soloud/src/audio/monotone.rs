use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

#[derive(Debug, AudioExt, LoadExt)]
pub struct Monotone {
    _inner: *mut ffi::Monotone,
}

impl Monotone {
    /// Set monotone parameters
    pub fn set_params(&mut self, hardware_channel: i32) -> Result<(), SoloudError> {
        ffi_call!(ffi::Monotone_setParams(self._inner, hardware_channel))
    }

    /// Set monotone parameters specifying the wave form
    pub fn set_params_ex(
        &mut self,
        hardware_channel: i32,
        wave_form: WaveForm,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Monotone_setParamsEx(
            self._inner,
            hardware_channel,
            wave_form as i32
        ))
    }
}
