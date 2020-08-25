use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Vic {
    _inner: *mut ffi::Vic,
}

impl Vic {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Vic_create() };
        assert!(!ptr.is_null());
        Vic { _inner: ptr }
    }
}

impl Drop for Vic {
    fn drop(&mut self) {
        unsafe {
            ffi::Vic_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
