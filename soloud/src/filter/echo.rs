use super::ParamType;
use crate::prelude::*;

/// Echo filter attributes
#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum EchoFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Delay attribute
    Delay,
    /// Decay attribute
    Decay,
    /// Filter attribute
    Filter,
}

/// Echo filter
#[derive(Debug, FilterExt)]
pub struct EchoFilter {
    _inner: *mut soloud_sys::soloud::EchoFilter,
}

impl EchoFilter {
    /// Set filter params
    pub fn set_params(&mut self, delay: f32) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::EchoFilter_setParams(self._inner, delay))
    }

    /// Set filter params with extra args
    pub fn set_params_ex(
        &mut self,
        delay: f32,
        decay: f32,
        filter: f32,
    ) -> Result<(), SoloudError> {
        ffi_call!(soloud_sys::soloud::EchoFilter_setParamsEx(
            self._inner,
            delay,
            decay,
            filter
        ))
    }
}
