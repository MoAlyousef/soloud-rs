use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// Wavstream audio type
#[derive(Debug)]
pub struct WavStream {
    inner: *mut ffi::WavStream,
}

crate::macros::load::impl_load_ext!(WavStream);
crate::macros::audio::impl_audio_ext!(WavStream);

impl WavStream {
    /// Get the length of the WavStream object
    pub fn length(&self) -> f64 {
        unsafe { ffi::WavStream_getLength(self.inner) }
    }

    /// Load a file to memory
    pub fn load_to_mem(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        let path = path
            .to_str()
            .ok_or(SoloudError::Internal(SoloudErrorKind::FileLoadFailed))?;
        let path = std::ffi::CString::new(path)?;
        ffi_call!(ffi::WavStream_loadToMem(self.inner, path.as_ptr()))
    }
}
