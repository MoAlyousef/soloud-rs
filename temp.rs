

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LofiFilterAttr {
LOFIFILTER_WET = 0,
LOFIFILTER_SAMPLERATE = 1,
LOFIFILTER_BITDEPTH = 2,
}

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BiquadResonantFilterAttr {
BIQUADRESONANTFILTER_LOWPASS = 0,
BIQUADRESONANTFILTER_HIGHPASS = 1,
BIQUADRESONANTFILTER_BANDPASS = 2,
BIQUADRESONANTFILTER_WET = 0,
BIQUADRESONANTFILTER_TYPE = 1,
BIQUADRESONANTFILTER_FREQUENCY = 2,
BIQUADRESONANTFILTER_RESONANCE = 3,
}

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum BassBoostFilterAttr{
BASSBOOSTFILTER_WET = 0,
BASSBOOSTFILTER_BOOST = 1,
}

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum RobotizeFilterAttr{
ROBOTIZEFILTER_WET = 0,
ROBOTIZEFILTER_FREQ = 1,
ROBOTIZEFILTER_WAVE = 2,
}

#[repr(u32)]
#[derive(FilterAttr, Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum WaveShaperFilterAttr {
WAVESHAPERFILTER_WET = 0,
WAVESHAPERFILTER_AMOUNT = 1,
}