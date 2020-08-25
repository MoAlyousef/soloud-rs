use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource, Loadable)]
pub struct WavStream {
    _inner: *mut ffi::WavStream,
}

impl WavStream {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::WavStream_create() };
        assert!(!ptr.is_null());
        WavStream { _inner: ptr }
    }

    pub fn get_length(&mut self) -> f64 {
        unsafe {
            ffi::WavStream_getLength(self._inner)
        }
    }
}

impl Drop for WavStream {
    fn drop(&mut self) {
        unsafe {
            ffi::WavStream_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
