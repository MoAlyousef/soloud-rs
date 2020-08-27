use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct Wav {
    _inner: *mut ffi::Wav,
}

impl Wav {
    
    pub fn load_raw_wav_8( &mut self, data: &mut [u8]) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave8(self._inner, data.as_mut_ptr(), data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn load_raw_wav_8_ex( &mut self, data: &mut [u8], samplerate: f32, channels: u32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave8Ex(self._inner, data.as_mut_ptr(), data.len() as u32, samplerate, channels);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn load_raw_wav_16( &mut self, data: &mut [i16]) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave16(self._inner, data.as_mut_ptr(), data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn load_raw_wav_16_ex( &mut self, data: &mut [i16], samplerate: f32, channels: u32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave16Ex(self._inner, data.as_mut_ptr(), data.len() as u32, samplerate, channels);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn load_raw_wav(&mut self, data: &mut [f32]) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWave(self._inner, data.as_mut_ptr(), data.len() as u32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn load_raw_wav_ex( &mut self, data: &mut [f32], samplerate: f32, channels: u32, copy: bool, take_ownership: bool) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Wav_loadRawWaveEx(self._inner, data.as_mut_ptr(), data.len() as u32, samplerate, channels, copy as i32, take_ownership as i32);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn length(&mut self) -> f64 {
        unsafe {
            ffi::Wav_getLength(self._inner)
        }
    }
}
