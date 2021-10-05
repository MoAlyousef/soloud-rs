use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Klatt wave forms
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum KlattWaveForm {
    /// Saw wave
    Saw = 0,
    /// Triangle wav
    Triangle,
    /// Sin wave
    Sin,
    /// Square wave
    Square,
    /// Pulse wave
    Pulse,
    /// Noise wave
    Noise,
    /// Warble wave
    Warble,
}

/// Speech audio source
#[derive(Debug)]
pub struct Speech {
    inner: *mut ffi::Speech,
}

crate::macros::audio::impl_audio_ext!(Speech);

impl Speech {
    /// set speech text
    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        let txt = std::ffi::CString::new(txt)?;
        ffi_call!(ffi::Speech_setText(self.inner, txt.as_ptr()))
    }

    /// Set speech params
    pub fn set_params(&mut self) -> Result<(), SoloudError> {
        ffi_call!(ffi::Speech_setParams(self.inner))
    }

    /// Set speech params
    pub fn set_params_ex(
        &mut self,
        base_freq: u32,
        base_speed: f32,
        base_declination: f32,
        base_waveform: KlattWaveForm,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Speech_setParamsEx(
            self.inner,
            base_freq,
            base_speed,
            base_declination,
            base_waveform as i32,
        ))
    }
}
