use crate::prelude::*;
use super::ParamType;

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

#[derive(FilterExt)]
pub struct BiquadResonantFilter {
    _inner: *mut soloud_sys::soloud::BiquadResonantFilter,
}

impl BiquadResonantFilter {
    pub fn set_params(&mut self, aType: BiquadResonantFilterType, aFrequency: f32, aResonance: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = soloud_sys::soloud::BiquadResonantFilter_setParams(self._inner, aType as i32, aFrequency, aResonance);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
}
