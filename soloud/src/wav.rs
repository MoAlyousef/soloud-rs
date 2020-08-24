use soloud_sys::soloud as ffi;
use crate::prelude::*;

pub struct Wav {
    _inner: *mut ffi::Wav,
}

impl Wav {
    pub fn default() -> Self {
        let ptr = unsafe {ffi::Wav_create() };
        assert!(!ptr.is_null());
        Wav { _inner: ptr }
    }

    pub fn load(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        unsafe {
            let path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
            let ret = ffi::Wav_load(self._inner, path.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::FailedToRun))
            } else {
                Ok(())
            }
        }
    }

    pub(crate) unsafe fn inner(&self) -> *mut ffi::Wav {
        self._inner
    }
}