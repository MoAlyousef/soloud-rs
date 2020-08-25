use soloud_sys::soloud as ffi;
use crate::prelude::*;

pub struct Speech {
    _inner: *mut ffi::Speech,
}

impl Speech {
    pub fn default() -> Self {
        let ptr = unsafe {ffi::Speech_create() };
        assert!(!ptr.is_null());
        Speech { _inner: ptr }
    }

    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        unsafe {
            let txt = std::ffi::CString::new(txt).unwrap();
            let ret = ffi::Speech_setText(self._inner, txt.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}

impl SoundSource for Speech {
    fn inner(&self) -> *mut *mut std::os::raw::c_void {
        self._inner as *mut *mut std::os::raw::c_void
    }
}