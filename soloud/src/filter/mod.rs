mod echo;
mod bass;
mod dc;
mod fft;
mod flanger;
mod freeverb;
mod lofi;
mod robotize;
mod waveshaper;
mod biquad;

pub use echo::*;
pub use bass::*;
pub use dc::*;
pub use fft::*;
pub use flanger::*;
pub use freeverb::*;
pub use lofi::*;
pub use robotize::*;
pub use waveshaper::*;
pub use biquad::*;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ParamType
{
    Float = 0,
    Int,
    Bool
}

