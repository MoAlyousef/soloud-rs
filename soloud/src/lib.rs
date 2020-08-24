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
                Err(SoloudError::Internal(SoloudErrorKind::FailedToRun))
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
            let ret = ffi::Soloud_play(self._inner, sound.inner());
            let msg = ffi::Soloud_getErrorString(self._inner, ret as i32);
            let msg = std::ffi::CStr::from_ptr(msg).to_str().unwrap();
            println!("{}", msg);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::FailedToRun))
            } else {
                Ok(())
            }
        }
    }
}