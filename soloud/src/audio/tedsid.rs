use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[derive(AudioExt, LoadExt)]
pub struct TedSid {
    _inner: *mut ffi::TedSid,
}
