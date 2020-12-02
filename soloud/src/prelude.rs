// pub use crate::effects::*;
use std::convert::From;
use std::os::raw::*;
use std::{fmt, io};
use std::path::Path;

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
    InvalidParameter = 1, // Some parameter is invalid
    FileNotFound = 2,     // File not found
    FileLoadFailed = 3,   // File found, but could not be loaded
    DllNotFound = 4,      // DLL not found, or wrong DLL
    OutOfMemory = 5,      // Out of memory
    NotImplemented = 6,   // Feature not implemented
    UnknownError = 7,     // Other error
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

bitflags! {
    pub struct SoloudFlag: i32 {
        const ClipRoundoff = 1;
        const EnableVisualization = 2;
        const LeftHanded3D = 4;
        const NoFpuRegisterChange = 8;
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

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Resampler {
    Point,
    Linear,
    Catmullrom,
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

pub unsafe trait AudioExt {
    /// Creates a default initialized object
    fn default() -> Self;
    /// Sets the volume
    fn set_volume(&mut self, volume: f32);
    /// Set whether the audio is looping
    fn set_looping(&mut self, flag: bool);
    /// Set auto stop
    fn set_auto_stop(&mut self, flag: bool);
    /// Set 3D min and max distances
    fn set_3d_min_max_distance(&mut self, min_distance: f32, max_distance: f32);
    /// Set 3D attenuation
    fn set_3d_attenuation(&mut self, model: AttenuationModel, rolloff_factor: f32);
    /// Set 3D doppler factor
    fn set_3d_doppler_factor(&mut self, doppler_factor: f32);
    /// Set 3D listener relative
    fn set_3d_listener_relative(&mut self, flag: bool);
    /// Set 3D distance delay
    fn set_3d_distance_delay(&mut self, delay: i32);
    // /// Set 3D collider
    // fn set_3d_collider(&mut self, collider: Option<&AudioCollider>);
    // /// Set 3D attenuator
    // fn set_3d_attenuator(&mut self, attenuator: Option<&AudioAttenuator>);
    /// Set inaudible behavior
    fn set_inaudible_behavior(&mut self, must_tick: bool, kill: bool);
    /// Set a loop point
    fn set_loop_point(&mut self, loop_point: f64);
    /// Get the loop point
    fn loop_point(&self) -> f64;
    /// Set a filter, the filter_id is assigned by the developer and becomes the id for that filter,
    /// and to cancel pass None as a filter to the already assigned id
    fn set_filter<F: FilterExt>(&mut self, filter_id: u32, filter: Option<&F>);
    /// Stop
    fn stop(&mut self);
    /// Get the inner pointer
    /// # Safety
    /// The inner pointer should be modified with care!
    unsafe fn inner(&self) -> *mut *mut c_void;
}

pub unsafe trait LoadExt {
    /// Load audio from a file
    fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SoloudError>;
    /// Load audio from memory
    fn load_mem(&mut self, data: &[u8]) -> Result<(), SoloudError>;
    /// Load audio from memory with options to copy and/or take ownership
    /// # Safety
    /// The audio source should not be invalidated
    unsafe fn load_mem_ex(
        &mut self,
        data: &[u8],
        copy: bool,
        take_ownership: bool,
    ) -> Result<(), SoloudError>;
}

pub unsafe trait FilterExt {
    /// Creates a default initialized object
    fn default() -> Self;
    /// Get the param count
    fn param_count(&mut self) -> i32;
    /// Get the param name by index
    fn param_name(&mut self, param_idx: u32) -> Option<String>;
    /// Get the param type by index
    fn param_type(&mut self, param_idx: u32) -> crate::filter::ParamType;
    /// Get the maximum value of a parameter
    fn param_max(&mut self, param_idx: u32) -> f32;
    /// Get the minimum value of a parameter
    fn param_min(&mut self, param_idx: u32) -> f32;
    /// Get the inner pointer
    /// # Safety
    /// The inner pointer should be modified with care!
    unsafe fn inner(&self) -> *mut *mut c_void;
}

pub trait FilterAttr {
    /// Convert a filter attribute to a u32
    fn to_u32(self) -> u32;
}

pub unsafe trait FromExt: Sized {
    /// Helper methods for loading an audio source from a path
    fn from_path<P: AsRef<Path>>(p: P) -> Result<Self, SoloudError>;
    /// Helper methods for loading an audio source from memory
    fn from_mem(data: &[u8]) -> Result<Self, SoloudError>;
}

unsafe impl<T: AudioExt + LoadExt> FromExt for T {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, SoloudError> {
        let mut x = Self::default();
        x.load(path.as_ref())?;
        Ok(x)
    }

    fn from_mem(data: &[u8]) -> Result<Self, SoloudError> {
        let mut x = Self::default();
        x.load_mem(data)?;
        Ok(x)
    }
}
