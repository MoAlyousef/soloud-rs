use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BiquadResonantFilterAttr {
    Wet = 0,
    Type,
    Frequency,
    Resonance,
}

#[repr(i32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BiquadResonantFilterType {
    LowPass,
    HighPass,
    BandPass,
}

#[derive(Debug, FilterExt)]
pub struct BiquadResonantFilter {
    _inner: *mut soloud_sys::soloud::BiquadResonantFilter,
}

impl BiquadResonantFilter {
    /// Set filter params
    pub fn set_params(
        &mut self,
        filter_type: BiquadResonantFilterType,
        frequency: f32,
        resonance: f32,
    ) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::BiquadResonantFilter_setParams(
            self._inner,
            filter_type as i32,
            frequency,
            resonance,
        ))
    }
}
