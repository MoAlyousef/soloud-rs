use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

#[derive(AudioExt, LoadExt)]
pub struct TedSid {
    _inner: *mut ffi::TedSid,
}
