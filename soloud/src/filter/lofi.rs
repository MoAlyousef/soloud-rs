use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct LofiFilter {
    _inner: *mut soloud_sys::soloud::LofiFilter,
}