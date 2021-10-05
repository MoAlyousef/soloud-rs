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
#[derive(Debug)]
pub struct Vic {
    inner: *mut ffi::Vic,
}

crate::macros::audio::impl_audio_ext!(Vic);

impl Vic {
    /// Get the VicModel of the Vic object
    pub fn model(&self) -> VicModel {
        unsafe { std::mem::transmute(ffi::Vic_getModel(self.inner)) }
    }

    /// Set the VicModel of the Vic object
    pub fn set_model(&mut self, model: VicModel) {
        unsafe { ffi::Vic_setModel(self.inner, model as i32) }
    }

    /// Get the VicRegister of the Vic object
    pub fn register(&self, reg: i32) -> VicRegister {
        unsafe { std::mem::transmute(ffi::Vic_getRegister(self.inner, reg)) }
    }

    /// Set the VicRegister of the Vic object
    pub fn set_register(&mut self, val: u8, reg: VicRegister) {
        unsafe { ffi::Vic_setRegister(self.inner, val as i32, reg as u8) }
    }
}
