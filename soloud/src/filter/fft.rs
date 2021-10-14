use super::ParamType;
use crate::prelude::*;

/// FFT Filter
#[derive(Debug)]
pub struct FFTFilter {
    inner: *mut soloud_sys::soloud::FFTFilter,
}

crate::macros::filter::impl_filter_ext!(FFTFilter);
