use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Noise types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NoiseType {
    /// White noise
    White = 0,
    /// Pink noise
    Pink,
    /// Brownish noise
    Brownish,
    /// Blueish noise
    Blueish,
}

/// Noise audio type
#[derive(Debug)]
pub struct Noise {
    inner: *mut ffi::Noise,
}

crate::macros::audio::impl_audio_ext!(Noise);

impl Noise {
    /// Set noise type
    pub fn set_type(&mut self, typ: NoiseType) {
        unsafe { ffi::Noise_setType(self.inner, typ as i32) }
    }

    /// Set the octave scale
    pub fn set_octave_scale(
        &mut self,
        oct_0: f32,
        oct_1: f32,
        oct_2: f32,
        oct_3: f32,
        oct_4: f32,
        oct_5: f32,
        oct_6: f32,
        oct_7: f32,
        oct_8: f32,
        oct_9: f32,
    ) {
        unsafe {
            ffi::Noise_setOctaveScale(
                self.inner, oct_0, oct_1, oct_2, oct_3, oct_4, oct_5, oct_6, oct_7, oct_8, oct_9,
            )
        }
    }
}
