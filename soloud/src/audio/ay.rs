use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Ay audio type
#[derive(Debug, AudioExt)]
pub struct Ay {
    _inner: *mut ffi::Ay,
}
