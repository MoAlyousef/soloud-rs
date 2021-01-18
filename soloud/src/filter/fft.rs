use super::ParamType;
use crate::prelude::*;

/// FFT Filter
#[derive(Debug, FilterExt)]
pub struct FFTFilter {
    _inner: *mut soloud_sys::soloud::FFTFilter,
}
