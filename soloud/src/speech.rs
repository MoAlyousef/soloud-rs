use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Speech {
    _inner: *mut ffi::Speech,
}

impl Speech {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Speech_create() };
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

    pub fn from_text(txt: &str) -> Result<Self, SoloudError> {
        let mut temp = Speech::default();
        if let Err(val) = temp.set_text(txt) {
            Err(val)
        } else {
            Ok(temp)
        }
    }
}

impl Drop for Speech {
    fn drop(&mut self) {
        unsafe {
            ffi::Speech_destroy(self._inner);
            self._inner = std::ptr::null_mut()
        }
    }
}
