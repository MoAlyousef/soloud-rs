use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

#[derive(Debug, AudioExt, LoadExt)]
pub struct Wav {
    _inner: *mut ffi::Wav,
}

impl Wav {
    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_8(&mut self, data: &[u8]) -> Result<(), SoloudError> {
        unsafe {
            let ret =
                ffi::Wav_loadRawWave8(self._inner, data.as_ptr() as *mut _, data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
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
        unsafe {
            let ret = ffi::Wav_loadRawWave8Ex(
                self._inner,
                data.as_ptr() as *mut _,
                data.len() as u32,
                samplerate,
                channels,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_16(&mut self, data: &[i16]) -> Result<(), SoloudError> {
        unsafe {
            let ret =
                ffi::Wav_loadRawWave16(self._inner, data.as_ptr() as *mut _, data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
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
        unsafe {
            let ret = ffi::Wav_loadRawWave16Ex(
                self._inner,
                data.as_ptr() as *mut _,
                data.len() as u32,
                samplerate,
                channels,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav(&mut self, data: &[f32]) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave(self._inner, data.as_ptr() as *mut _, data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
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
        unsafe {
            let ret = ffi::Wav_loadRawWaveEx(
                self._inner,
                data.as_ptr() as *mut _,
                data.len() as u32,
                samplerate,
                channels,
                copy as i32,
                take_ownership as i32,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Get the length of the Wav object
    pub fn length(&self) -> f64 {
        unsafe { ffi::Wav_getLength(self._inner) }
    }
}
