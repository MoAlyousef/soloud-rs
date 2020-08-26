use crate::prelude::*;
use super::ParamType;


#[derive(FilterExt)]
pub struct BiquadResonantFilter {
    _inner: *mut soloud_sys::soloud::BiquadResonantFilter,
}
