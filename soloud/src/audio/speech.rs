use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Speech {
    _inner: *mut ffi::Speech,
}

impl Speech {
    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
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
