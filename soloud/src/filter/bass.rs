use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct BassboostFilter {
    _inner: *mut soloud_sys::soloud::BassboostFilter,
}