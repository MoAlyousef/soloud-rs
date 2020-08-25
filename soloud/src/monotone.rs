use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource, Loadable)]
pub struct Monotone {
    _inner: *mut ffi::Monotone,
}

impl Monotone {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Monotone_create() };
        assert!(!ptr.is_null());
        Monotone { _inner: ptr }
    }
}

impl Drop for Monotone {
    fn drop(&mut self) {
        unsafe {
            ffi::Monotone_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
