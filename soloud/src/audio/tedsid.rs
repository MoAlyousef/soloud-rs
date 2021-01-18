use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// TedSid audio source
#[derive(Debug, AudioExt, LoadExt)]
pub struct TedSid {
    _inner: *mut ffi::TedSid,
}
