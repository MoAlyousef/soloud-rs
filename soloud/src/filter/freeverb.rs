use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct FreeverbFilter {
    _inner: *mut soloud_sys::soloud::FreeverbFilter,
}
