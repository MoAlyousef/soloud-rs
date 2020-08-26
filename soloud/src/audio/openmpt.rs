use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct Openmpt {
    _inner: *mut ffi::Openmpt,
}
