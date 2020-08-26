use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Sfxr {
    _inner: *mut ffi::Sfxr,
}
