use crate::prelude::*;
use super::ParamType;

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum RobotizeFilterAttr{
    Wet = 0,
    Freq = 1,
    Wave = 2,
}

#[derive(FilterExt)]
pub struct RobotizeFilter {
    _inner: *mut soloud_sys::soloud::RobotizeFilter,
}

impl RobotizeFilter {
    pub fn set_params(&mut self, aFreq: f32, aWaveForm: WaveForm) {
        unsafe {
            soloud_sys::soloud::RobotizeFilter_setParams(self._inner, aFreq, aWaveForm as i32)
        }
    }
}