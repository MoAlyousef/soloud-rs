use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Noise {
    _inner: *mut ffi::Noise,
}

impl Noise {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Noise_create() };
        assert!(!ptr.is_null());
        Noise { _inner: ptr }
    }
}

impl Drop for Noise {
    fn drop(&mut self) {
        unsafe {
            ffi::Noise_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
