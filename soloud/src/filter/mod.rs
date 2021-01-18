//! Audio filters

mod bass;
mod biquad;
mod dc;
mod echo;
mod fft;
mod flanger;
mod freeverb;
mod lofi;
mod robotize;
mod waveshaper;

pub use bass::*;
pub use biquad::*;
pub use dc::*;
pub use echo::*;
pub use fft::*;
pub use flanger::*;
pub use freeverb::*;
pub use lofi::*;
pub use robotize::*;
pub use waveshaper::*;

/// The associated parameter types
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ParamType {
    /// Floating-point parameter type
    Float = 0,
    /// Integral parameter type
    Int,
    /// Boolean parameter type
    Bool,
}
