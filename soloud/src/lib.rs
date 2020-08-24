pub mod prelude;
pub mod wav;

pub use prelude::*;

use soloud_sys::soloud as ffi;

pub struct Soloud {
    _inner: *mut ffi::Soloud,
}

impl Soloud {
    pub fn default() -> Self {
        unsafe {
            let ptr = ffi::Soloud_create();
            assert!(!ptr.is_null());
            Soloud { _inner: ptr }
        }
    }

    pub fn init(&mut self) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_init(self._inner);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn deinit(&mut self) {
        unsafe { ffi::Soloud_deinit(self._inner) }
    }

    pub fn play(&self, sound: &crate::wav::Wav) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_play(self._inner, sound.inner()) as i32;
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}