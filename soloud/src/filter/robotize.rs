use super::ParamType;
use crate::prelude::*;

/// Robotize filter attributes
#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum RobotizeFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Frequency attribute
    Freq = 1,
    /// Wave attribute
    Wave = 2,
}

/// Robotize filter
#[derive(Debug, FilterExt)]
pub struct RobotizeFilter {
    _inner: *mut soloud_sys::soloud::RobotizeFilter,
}

impl RobotizeFilter {
    /// Set filter params
    pub fn set_params(&mut self, freq: f32, wave_form: WaveForm) {
        unsafe { soloud_sys::soloud::RobotizeFilter_setParams(self._inner, freq, wave_form as i32) }
    }
}
