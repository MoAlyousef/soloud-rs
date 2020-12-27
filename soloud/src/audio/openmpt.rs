use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

#[derive(Debug, AudioExt, LoadExt)]
pub struct Openmpt {
    _inner: *mut ffi::Openmpt,
}
