use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Sfxr {
    _inner: *mut ffi::Sfxr,
}

impl Sfxr {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Sfxr_create() };
        assert!(!ptr.is_null());
        Sfxr { _inner: ptr }
    }
}

impl Drop for Sfxr {
    fn drop(&mut self) {
        unsafe {
            ffi::Sfxr_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
