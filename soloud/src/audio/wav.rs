use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// Wav audio type
#[derive(Debug)]
pub struct Wav {
    inner: *mut ffi::Wav,
}

crate::macros::load::impl_load_ext!(Wav);
crate::macros::audio::impl_audio_ext!(Wav);

impl Wav {
    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_8(&mut self, data: &[u8]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave8(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_8_ex(
        &mut self,
        data: &[u8],
        samplerate: f32,
        channels: u32,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave8Ex(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_16(&mut self, data: &[i16]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave16(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_16_ex(
        &mut self,
        data: &[i16],
        samplerate: f32,
        channels: u32,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave16Ex(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav(&mut self, data: &[f32]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_ex(
        &mut self,
        data: &[f32],
        samplerate: f32,
        channels: u32,
        copy: bool,
        take_ownership: bool,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWaveEx(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
            copy as i32,
            take_ownership as i32,
        ))
    }

    /// Get the length of the Wav object
    pub fn length(&self) -> f64 {
        unsafe { ffi::Wav_getLength(self.inner) }
    }
}
