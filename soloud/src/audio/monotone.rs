use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct Monotone {
    _inner: *mut ffi::Monotone,
}