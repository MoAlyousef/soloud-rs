use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicModel {
    Pal = 0,
    Ntsc,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicRegister {
    Bass = 0,
    Alto,
    Soprano,
    Noise,
    MaxRegs,
}

#[derive(AudioExt)]
pub struct Vic {
    _inner: *mut ffi::Vic,
}

impl Vic {
    /// Get the VicModel of the Vic object
    pub fn model(&self) -> VicModel {
        assert!(!self._inner.is_null());
        unsafe { std::mem::transmute(ffi::Vic_getModel(self._inner)) }
    }
    
    /// Set the VicModel of the Vic object
    pub fn set_model(&mut self, model: VicModel) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Vic_setModel(self._inner, model as i32) }
    }

    /// Get the VicRegister of the Vic object
    pub fn register(&self, reg: i32) -> VicRegister {
        assert!(!self._inner.is_null());
        unsafe { std::mem::transmute(ffi::Vic_getRegister(self._inner, reg)) }
    }
    
    /// Set the VicRegister of the Vic object
    pub fn set_register(&mut self, val: u8, reg: VicRegister) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Vic_setRegister(self._inner, val as i32, reg as u8) }
    }
}
