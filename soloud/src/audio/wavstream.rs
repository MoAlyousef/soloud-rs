use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
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
        unsafe {
            let path = path.to_str().unwrap();
            let path = std::ffi::CString::new(path)?;
            let ret = ffi::WavStream_loadToMem(self._inner, path.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
