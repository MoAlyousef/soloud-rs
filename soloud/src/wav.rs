use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioSource)]
pub struct Wav {
    _inner: *mut ffi::Wav,
}

impl Wav {
    pub fn default() -> Self {
        let ptr = unsafe { ffi::Wav_create() };
        assert!(!ptr.is_null());
        Wav { _inner: ptr }
    }

    pub fn load(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
        unsafe {
            let path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
            let ret = ffi::Wav_load(self._inner, path.as_ptr());
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn from_path(path: &std::path::Path) -> Result<Self, SoloudError> {
        let mut temp = Wav::default();
        if let Err(val) = temp.load(path) {
            Err(val)
        } else {
            Ok(temp)
        }
    }
}

impl Drop for Wav {
    fn drop(&mut self) {
        unsafe {
            ffi::Wav_destroy(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }
}
