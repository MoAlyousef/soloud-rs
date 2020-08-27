use std::convert::From;
use std::{fmt, io};

#[derive(Debug)]
pub enum SoloudError {
    IoError(io::Error),
    NullError(std::ffi::NulError),
    Internal(SoloudErrorKind),
    Unknown(String),
}

/// Error kinds enum for SoloudError
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SoloudErrorKind {
    InvalidParemeter = 1, // Some parameter is invalid
    FileNotFound = 2,    // File not found
    FileLoadFailed = 3,  // File found, but could not be loaded
    DllNotFound = 4,     // DLL not found, or wrong DLL
    OutOfMemory = 5,     // Out of memory
    NotImplemented = 6,  // Feature not implemented
    UnknownError = 7,    // Other error
}

impl SoloudErrorKind {
    pub fn from_i32(val: i32) -> SoloudErrorKind {
        unsafe { std::mem::transmute(val) }
    }
}

impl std::error::Error for SoloudError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SoloudError::IoError(err) => Some(err),
            SoloudError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SoloudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SoloudError::IoError(ref err) => err.fmt(f),
            SoloudError::NullError(ref err) => err.fmt(f),
            SoloudError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
            SoloudError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for SoloudError {
    fn from(err: io::Error) -> SoloudError {
        SoloudError::IoError(err)
    }
}

impl From<std::ffi::NulError> for SoloudError {
    fn from(err: std::ffi::NulError) -> SoloudError {
        SoloudError::NullError(err)
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum SoloudFlags
{
    ClipRoundoff = 1,
    EnableVisualization = 2,
    LeftHanded3D = 4,
    NoFpuRegisterChange = 8
}

impl std::ops::BitOr for SoloudFlags {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self {
        unsafe { std::mem::transmute(self as u32 | rhs as u32) }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum WaveForm {
    Square = 0,
    Saw,
    Sin,
    Triangle,
    Bounce,
    Jaws,
    Humps,
    FSquare,
    FSaw,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum AttenuationModel {
    // No attenuation
    NoAttenuation = 0,
    // Inverse distance attenuation model
    InverseDistance = 1,
    // Linear distance attenuation model
    LinearDistance = 2,
    // Exponential distance attenuation model
    ExponentialDistance = 3,
}

pub struct Filter {
    _inner: *mut soloud_sys::soloud::Filter,
}

impl Filter {
    pub fn inner(&self) -> *mut soloud_sys::soloud::Filter {
        self._inner
    }
}

pub struct AudioCollider {
    _inner: *mut soloud_sys::soloud::AudioCollider,
}

impl AudioCollider {
    pub fn inner(&self) -> *mut soloud_sys::soloud::AudioCollider {
        self._inner
    }
}

pub struct AudioAttenuator {
    _inner: *mut soloud_sys::soloud::AudioAttenuator,
}

impl AudioAttenuator {
    pub fn inner(&self) -> *mut soloud_sys::soloud::AudioAttenuator {
        self._inner
    }
}

pub unsafe trait AudioExt {
    fn default() -> Self;

    fn set_volume(&mut self, volume: f32);

    fn set_looping(&mut self, flag: bool);

    fn set_auto_stop(&mut self, flag: bool);

    fn set_3d_min_max_distance(&mut self, min_distance: f32, max_distance: f32);

    fn set_3d_attenuation(
        &mut self,
        model: AttenuationModel,
        rolloff_factor: f32,
    );

    fn set_3d_doppler_factor(&mut self, doppler_factor: f32);

    fn set_3d_listener_relative(&mut self, flag: bool);

    fn set_3d_distance_delay(&mut self, delay: i32);

    fn set_3d_collider(&mut self, collider: Option<&AudioCollider>);

    fn set_3d_attenuator(&mut self, attenuator: Option<&AudioAttenuator>);

    fn set_inaudible_behavior(&mut self, must_tick: bool, kill: bool);

    fn set_loop_point(&mut self, loop_point: f64);

    fn loop_point(&self) -> f64;

    fn set_filter<F: FilterExt>(&mut self, filter_id: u32, filter: Option<&F>);

    fn stop(&mut self);

    fn inner(&self) -> *mut *mut std::os::raw::c_void;
}

pub unsafe trait LoadExt {
    fn load(&mut self, path: &std::path::Path) -> Result<(), SoloudError>;

    fn load_mem( &mut self, data: &[u8]) -> Result<(), SoloudError>;

    fn load_mem_ex( &mut self, data: &[u8], copy: bool, take_ownershipt: bool) -> Result<(), SoloudError>;
}

pub unsafe trait FilterExt {
    fn inner(&self) -> *mut *mut std::os::raw::c_void;
    
    fn default() -> Self;
    
    fn param_count(&mut self) -> i32;

    fn param_name( &mut self, param_idx: u32) -> Option<String>;

    fn param_type( &mut self, param_idx: u32) -> crate::filter::ParamType;

    fn param_max(&mut self, param_idx: u32) -> f32;

    fn param_min(&mut self, param_idx: u32) -> f32;

}

pub trait FilterAttr {
    fn to_u32(self) -> u32;
}