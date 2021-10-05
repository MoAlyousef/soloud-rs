use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// OpenMPT audio type
#[derive(Debug)]
pub struct Openmpt {
    inner: *mut ffi::Openmpt,
}

crate::macros::load::impl_load_ext!(Openmpt);
crate::macros::audio::impl_audio_ext!(Openmpt);