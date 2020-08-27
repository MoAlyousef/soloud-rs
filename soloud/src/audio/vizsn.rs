use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Vizsn {
    _inner: *mut ffi::Vizsn,
}

impl Vizsn {
    /// set speech text
    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        unsafe {
            let txt = std::ffi::CString::new(txt)?;
            ffi::Vizsn_setText(self._inner, txt.as_ptr() as *mut _);
            Ok(())
        }
    }
}
