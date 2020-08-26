#![allow(non_snake_case)]

pub mod prelude;
pub mod speech;
pub mod wav;
pub mod ay;
pub mod monotone;
pub mod noise;
pub mod openmpt;
pub mod queue;
pub mod sfxr;
pub mod tedsid;
pub mod vic;
pub mod vizsn;
pub mod wavstream;

#[macro_use]
extern crate soloud_derive;

pub use prelude::*;

use soloud_sys::soloud as ffi;

pub type Handle = u32;

#[derive(Clone)]
pub struct Soloud {
    _inner: *mut ffi::Soloud,
}

unsafe impl Send for Soloud {}

unsafe impl Sync for Soloud {}

impl Soloud {
    pub fn default_uninit() -> std::mem::MaybeUninit<Self> {
        unsafe {
            let ptr = ffi::Soloud_create();
            assert!(!ptr.is_null());
            std::mem::MaybeUninit::new(Soloud { _inner: ptr })
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

    pub fn default() -> Result<Self, SoloudError> {
        let mut temp = unsafe { Soloud::default_uninit().assume_init() };
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
        assert!(!self._inner.is_null());
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
