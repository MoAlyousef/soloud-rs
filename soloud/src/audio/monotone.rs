use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct Monotone {
    _inner: *mut ffi::Monotone,
}

impl Monotone {
    /// Set monotone parameters
    pub fn set_params(&mut self, hardware_channel: i32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Monotone_setParams(self._inner, hardware_channel);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Set monotone parameters specifying the wave form
    pub fn set_params_ex(
        &mut self,
        hardware_channel: i32,
        wave_form: WaveForm,
    ) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Monotone_setParamsEx(self._inner, hardware_channel, wave_form as i32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
