use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Sfxr presets
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SfxrPreset {
    /// Coin preset
    Coin = 0,
    /// Laser preset
    Laser = 1,
    /// Explosion preset
    Explosion = 2,
    /// PowerUp preset
    PowerUp = 3,
    /// Hurt preset
    Hurt = 4,
    /// Jump preset
    Jump = 5,
    /// Blip preset
    Blip = 6,
}

/// Sfxr audio type
#[derive(Debug)]
pub struct Sfxr {
    inner: *mut ffi::Sfxr,
}

crate::macros::audio::impl_audio_ext!(Sfxr);

impl Sfxr {
    /// Load preset
    pub fn load_preset(&mut self, preset: SfxrPreset, rand_seed: i32) -> Result<(), SoloudError> {
        ffi_call!(ffi::Sfxr_loadPreset(self.inner, preset as i32, rand_seed))
    }

    /// Resets parameters
    pub fn reset_params(&mut self) {
        unsafe { ffi::Sfxr_resetParams(self.inner) }
    }

    /// Load parameters from a file
    pub fn load_params(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        let path = path
            .to_str()
            .ok_or(SoloudError::Internal(SoloudErrorKind::FileLoadFailed))?;
        let path = std::ffi::CString::new(path)?;
        ffi_call!(ffi::Sfxr_loadParams(self.inner, path.as_ptr()))
    }

    /// Load parameters from memory
    pub fn load_params_mem(&mut self, params: &[u8]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Sfxr_loadParamsMemEx(
            self.inner,
            params.as_ptr() as *mut _,
            params.len() as u32,
            1,
            1,
        ))
    }
}
