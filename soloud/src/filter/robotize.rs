use super::ParamType;
use crate::prelude::*;

/// Robotize filter attributes
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum RobotizeFilterAttr {
    /// Wet attribute
    Wet = 0,
    /// Frequency attribute
    Freq = 1,
    /// Wave attribute
    Wave = 2,
}

/// Robotize filter
#[derive(Debug)]
pub struct RobotizeFilter {
    inner: *mut soloud_sys::soloud::RobotizeFilter,
}

crate::macros::filter::impl_filter_ext!(RobotizeFilter);
crate::macros::filter::impl_filter_type!(RobotizeFilterAttr);

impl RobotizeFilter {
    /// Set filter params
    pub fn set_params(&mut self, freq: f32, wave_form: WaveForm) {
        unsafe { soloud_sys::soloud::RobotizeFilter_setParams(self.inner, freq, wave_form as i32) }
    }
}
