use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Vic model
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicModel {
    /// Pal model
    Pal = 0,
    /// Ntsc model
    Ntsc,
}

/// Vic register
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicRegister {
    /// Bass register
    Bass = 0,
    /// Alto register
    Alto,
    /// Soprano register
    Soprano,
    /// Noise register
    Noise,
    /// Max regs register
    MaxRegs,
}

/// Vic-20 emulator audio type
#[derive(Debug, AudioExt)]
pub struct Vic {
    _inner: *mut ffi::Vic,
}

impl Vic {
    /// Get the VicModel of the Vic object
    pub fn model(&self) -> VicModel {
        unsafe { std::mem::transmute(ffi::Vic_getModel(self._inner)) }
    }

    /// Set the VicModel of the Vic object
    pub fn set_model(&mut self, model: VicModel) {
        unsafe { ffi::Vic_setModel(self._inner, model as i32) }
    }

    /// Get the VicRegister of the Vic object
    pub fn register(&self, reg: i32) -> VicRegister {
        unsafe { std::mem::transmute(ffi::Vic_getRegister(self._inner, reg)) }
    }

    /// Set the VicRegister of the Vic object
    pub fn set_register(&mut self, val: u8, reg: VicRegister) {
        unsafe { ffi::Vic_setRegister(self._inner, val as i32, reg as u8) }
    }
}
