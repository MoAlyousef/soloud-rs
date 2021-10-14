use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// TedSid audio source
#[derive(Debug)]
pub struct TedSid {
    inner: *mut ffi::TedSid,
}

crate::macros::load::impl_load_ext!(TedSid);
crate::macros::audio::impl_audio_ext!(TedSid);
