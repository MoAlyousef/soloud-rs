use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// Monotone audio type
#[derive(Debug)]
pub struct Monotone {
    inner: *mut ffi::Monotone,
}

crate::macros::load::impl_load_ext!(Monotone);
crate::macros::audio::impl_audio_ext!(Monotone);

impl Monotone {
    /// Set monotone parameters
    pub fn set_params(&mut self, hardware_channel: i32) -> Result<(), SoloudError> {
        ffi_call!(ffi::Monotone_setParams(self.inner, hardware_channel))
    }

    /// Set monotone parameters specifying the wave form
    pub fn set_params_ex(
        &mut self,
        hardware_channel: i32,
        wave_form: WaveForm,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Monotone_setParamsEx(
            self.inner,
            hardware_channel,
            wave_form as i32
        ))
    }
}
