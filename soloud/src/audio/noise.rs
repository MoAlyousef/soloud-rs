use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Noise {
    _inner: *mut ffi::Noise,
}
