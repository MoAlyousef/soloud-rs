use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Ay {
    _inner: *mut ffi::Ay,
}

impl Ay {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Ay_create() };
        assert!(!ptr.is_null());
        Ay { _inner: ptr }
    }
}

impl Drop for Ay {
    fn drop(&mut self) {
        unsafe {
            ffi::Ay_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
