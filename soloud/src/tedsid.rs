use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource, Loadable)]
pub struct TedSid {
    _inner: *mut ffi::TedSid,
}

impl TedSid {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::TedSid_create() };
        assert!(!ptr.is_null());
        TedSid { _inner: ptr }
    }
}

impl Drop for TedSid {
    fn drop(&mut self) {
        unsafe {
            ffi::TedSid_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
