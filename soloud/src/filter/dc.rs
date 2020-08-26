use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct DCRemovalFilter {
    _inner: *mut soloud_sys::soloud::DCRemovalFilter,
}