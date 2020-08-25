#![allow(non_snake_case)]

pub mod prelude;
pub mod speech;
pub mod wav;

#[macro_use]
extern crate soloud_derive;

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

    pub fn new() -> Result<Self, SoloudError> {
        let mut temp = Soloud::default();
        if let Err(val) = temp.init() {
            Err(val)
        } else {
            Ok(temp)
        }
    }

    pub fn deinit(&mut self) {
        unsafe {
            ffi::Soloud_deinit(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }

    pub fn play<T: AudioSource>(&self, sound: &T) -> Handle {
        unsafe { ffi::Soloud_play(self._inner, sound.inner()) }
    }

    pub fn get_active_voice_count(&self) -> u32 {
        unsafe { ffi::Soloud_getActiveVoiceCount(self._inner) }
    }

    pub fn get_voice_count(&self) -> u32 {
        unsafe { ffi::Soloud_getVoiceCount(self._inner) }
    }

    pub fn set_global_volume(&mut self, val: f32) {
        unsafe { ffi::Soloud_setGlobalVolume(self._inner, val) }
    }
}
