use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Vic {
    _inner: *mut ffi::Vic,
}