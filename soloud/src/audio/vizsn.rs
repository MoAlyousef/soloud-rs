use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Vizsn audio type
#[derive(Debug)]
pub struct Vizsn {
    inner: *mut ffi::Vizsn,
}

crate::macros::audio::impl_audio_ext!(Vizsn);

impl Vizsn {
    /// set speech text
    pub fn set_text(&mut self, txt: &str) -> Result<(), SoloudError> {
        unsafe {
            let txt = std::ffi::CString::new(txt)?;
            ffi::Vizsn_setText(self.inner, txt.as_ptr() as *mut _);
            Ok(())
        }
    }
}
