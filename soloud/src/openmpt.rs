use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource, Loadable)]
pub struct Openmpt {
    _inner: *mut ffi::Openmpt,
}

impl Openmpt {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Openmpt_create() };
        assert!(!ptr.is_null());
        Openmpt { _inner: ptr }
    }
}

impl Drop for Openmpt {
    fn drop(&mut self) {
        unsafe {
            ffi::Openmpt_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
