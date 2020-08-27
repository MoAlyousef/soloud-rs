use super::ParamType;
use crate::prelude::*;

#[derive(FilterExt)]
pub struct FFTFilter {
    _inner: *mut soloud_sys::soloud::FFTFilter,
}
