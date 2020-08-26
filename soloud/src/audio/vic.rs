use crate::prelude::*;
use soloud_sys::soloud as ffi;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicModel {
    Pal	= 0,
    Ntsc
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum VicRegister {
    Bass = 0,
    Alto,
    Soprano,
    Noise,
    MaxRegs
}

#[derive(AudioExt)]
pub struct Vic {
    _inner: *mut ffi::Vic,
}

impl Vic {
    pub fn model(&self) -> VicModel {
        unsafe {
            std::mem::transmute(ffi::Vic_getModel(self._inner))
        }
    }
    pub fn set_model(&mut self, model: VicModel) {
        unsafe {
            ffi::Vic_setModel(self._inner, model as i32)
        }
    }
    pub fn register(&self, reg: i32) -> VicRegister {
        unsafe {
            std::mem::transmute(ffi::Vic_getRegister(self._inner, reg))
        }
    }
    pub fn set_register(&mut self, val: u8, reg: VicRegister) {
        unsafe {
            ffi::Vic_setRegister(self._inner, val as i32, reg as u8)
        }
    }
}