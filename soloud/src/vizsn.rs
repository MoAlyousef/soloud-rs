use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Vizsn {
    _inner: *mut ffi::Vizsn,
}

impl Vizsn {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Vizsn_create() };
        assert!(!ptr.is_null());
        Vizsn { _inner: ptr }
    }

    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        unsafe {
            let txt = std::ffi::CString::new(txt)?;
            ffi::Vizsn_setText(self._inner, txt.as_ptr() as *mut _);
            Ok(())
        }
    }
}

impl Drop for Vizsn {
    fn drop(&mut self) {
        unsafe {
            ffi::Vizsn_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
