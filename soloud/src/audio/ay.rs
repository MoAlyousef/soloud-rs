use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Ay audio type
#[derive(Debug)]
pub struct Ay {
    inner: *mut ffi::Ay,
}

crate::macros::audio::impl_audio_ext!(Ay);
