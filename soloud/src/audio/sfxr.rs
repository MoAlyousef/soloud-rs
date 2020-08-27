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
    pub fn load_preset(&mut self, preset: SfxrPreset, rand_seed: i32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = ffi::Sfxr_loadPreset(self._inner, preset as i32, rand_seed);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
