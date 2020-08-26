use crate::prelude::*;
use super::ParamType;

#[derive(FilterExt)]
pub struct RobotizeFilter {
    _inner: *mut soloud_sys::soloud::RobotizeFilter,
}