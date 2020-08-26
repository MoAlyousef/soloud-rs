use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct FFTFilter {
    _inner: *mut soloud_sys::soloud::FFTFilter,
}