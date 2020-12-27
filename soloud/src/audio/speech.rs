use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum KlattWaveForm {
    Saw = 0,
    Triangle,
    Sin,
    Square,
    Pulse,
    Noise,
    Warble,
}

#[derive(Debug, AudioExt)]
pub struct Speech {
    _inner: *mut ffi::Speech,
}

impl Speech {
    /// set speech text
    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        unsafe {
            let txt = std::ffi::CString::new(txt)?;
            let ret = ffi::Speech_setText(self._inner, txt.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Set speech params
    pub fn set_params(&mut self) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Speech_setParams(self._inner);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Set speech params
    pub fn set_params_ex(
        &mut self,
        base_freq: u32,
        base_speed: f32,
        base_declination: f32,
        base_waveform: KlattWaveForm,
    ) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Speech_setParamsEx(
                self._inner,
                base_freq,
                base_speed,
                base_declination,
                base_waveform as i32,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
