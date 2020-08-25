use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Queue {
    _inner: *mut ffi::Queue,
}

impl Queue {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Queue_create() };
        assert!(!ptr.is_null());
        Queue { _inner: ptr }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            ffi::Queue_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
