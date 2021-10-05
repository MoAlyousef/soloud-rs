use super::ParamType;
use crate::prelude::*;

/// Biquad resonant filter attributes
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BiquadResonantFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Type attribute
    Type,
    /// Frequency attribute
    Freq,
    /// Resoncance attribute
    Resonance,
}

/// Biquad resonant filter types
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BiquadResonantFilterType {
    /// Lowpass biquad resonant filter
    LowPass,
    /// Highpass biquad resonant filter
    HighPass,
    /// Bandpass biquad resonant filter
    BandPass,
}

/// Biquad resonant filter
#[derive(Debug)]
pub struct BiquadResonantFilter {
    inner: *mut soloud_sys::soloud::BiquadResonantFilter,
}

crate::macros::filter::impl_filter_ext!(BiquadResonantFilter);
crate::macros::filter::impl_filter_type!(BiquadResonantFilterType);
crate::macros::filter::impl_filter_type!(BiquadResonantFilterAttr);

impl BiquadResonantFilter {
    /// Set filter params
    pub fn set_params(
        &mut self,
        filter_type: BiquadResonantFilterType,
        frequency: f32,
        resonance: f32,
    ) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::BiquadResonantFilter_setParams(
            self.inner,
            filter_type as i32,
            frequency,
            resonance,
        ))
    }
}
