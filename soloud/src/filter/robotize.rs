use super::ParamType;
use crate::prelude::*;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum RobotizeFilterAttr {
    Wet = 0,
    Freq = 1,
    Wave = 2,
}

#[derive(FilterExt)]
pub struct RobotizeFilter {
    _inner: *mut soloud_sys::soloud::RobotizeFilter,
}

impl RobotizeFilter {
    pub fn set_params(&mut self, freq: f32, wave_form: WaveForm) {
        assert!(!self._inner.is_null());
        unsafe { soloud_sys::soloud::RobotizeFilter_setParams(self._inner, freq, wave_form as i32) }
    }
}
