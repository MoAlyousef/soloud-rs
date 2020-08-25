pub mod prelude;
pub mod wav;
pub mod speech;

pub use prelude::*;

use soloud_sys::soloud as ffi;

pub type Handle = u32;

pub struct Soloud {
    _inner: *mut ffi::Soloud,
}

impl Soloud {
    pub fn default() -> Self {
        unsafe {
            let ptr = ffi::Soloud_create();
            assert!(!ptr.is_null());
            Soloud { _inner: ptr }
        }
    }

    pub fn init(&mut self) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_init(self._inner);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn deinit(&mut self) {
        unsafe { ffi::Soloud_deinit(self._inner) }
    }

    pub fn play<T: SoundSource>(&self, sound: &T) -> Handle {
        unsafe {
            ffi::Soloud_play(self._inner, sound.inner())
        }
    }

    pub fn get_active_voice_count(&self) -> u32 {
        unsafe {
            ffi::Soloud_getActiveVoiceCount(self._inner)
        }
    }

    pub fn get_voice_count(&self) -> u32 {
        unsafe {
            ffi::Soloud_getVoiceCount(self._inner)
        }
    }

    pub fn set_global_volume(&mut self, val: f32) {
        unsafe {
            ffi::Soloud_setGlobalVolume(self._inner, val)
        }
    }
}