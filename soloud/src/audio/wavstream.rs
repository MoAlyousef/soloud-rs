use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

#[derive(Debug, AudioExt, LoadExt)]
pub struct WavStream {
    _inner: *mut ffi::WavStream,
}

impl WavStream {
    /// Get the length of the WavStream object
    pub fn length(&self) -> f64 {
        unsafe { ffi::WavStream_getLength(self._inner) }
    }

    /// Load a file to memory
    pub fn load_to_mem(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        let path = path
            .to_str()
            .ok_or(SoloudError::Internal(SoloudErrorKind::FileLoadFailed))?;
        let path = std::ffi::CString::new(path)?;
        ffi_call!(ffi::WavStream_loadToMem(self._inner, path.as_ptr()))
    }
}
