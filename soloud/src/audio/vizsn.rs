use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Vizsn {
    _inner: *mut ffi::Vizsn,
}

impl Vizsn {

    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        unsafe {
            let txt = std::ffi::CString::new(txt)?;
            ffi::Vizsn_setText(self._inner, txt.as_ptr() as *mut _);
            Ok(())
        }
    }
}
