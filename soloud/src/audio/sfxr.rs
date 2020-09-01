use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SfxrPreset {
    Coin = 0,
    Laser = 1,
    Explosion = 2,
    PowerUp = 3,
    Hurt = 4,
    Jump = 5,
    Blip = 6,
}

#[derive(AudioExt)]
pub struct Sfxr {
    _inner: *mut ffi::Sfxr,
}

impl Sfxr {
    /// Load preset
    pub fn load_preset(&mut self, preset: SfxrPreset, rand_seed: i32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Sfxr_loadPreset(self._inner, preset as i32, rand_seed);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Resets parameters
    pub fn reset_params(&mut self) {
        unsafe { ffi::Sfxr_resetParams(self._inner) }
    }

    /// Load parameters from a file
    pub fn load_params(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        unsafe {
            let path = path.to_str().ok_or(SoloudError::Internal(SoloudErrorKind::FileLoadFailed))?;
            let path = std::ffi::CString::new(path)?;
            let ret = ffi::Sfxr_loadParams(self._inner, path.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Load parameters from memory
    pub fn load_params_mem(&mut self, params: &[u8]) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Sfxr_loadParamsMemEx(
                self._inner,
                params.as_ptr() as *mut _,
                params.len() as u32,
                1,
                1,
            );
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
