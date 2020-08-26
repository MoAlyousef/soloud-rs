use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt)]
pub struct Queue {
    _inner: *mut ffi::Queue,
}
