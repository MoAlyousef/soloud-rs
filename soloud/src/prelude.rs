// pub use crate::effects::*;
use std::convert::From;
use std::os::raw::*;
use std::path::Path;
use std::{fmt, io};

/// Soloud error types
#[derive(Debug)]
pub enum SoloudError {
    /// i/o error
    IoError(io::Error),
    /// Mull value error in the read memory
    NullError(std::ffi::NulError),
    /// Internal soloud error
    Internal(SoloudErrorKind),
    /// Unknown error
    Unknown(String),
}

/// Error kinds enum for SoloudError
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SoloudErrorKind {
    /// Some parameter is invalid
    InvalidParameter = 1,
    /// File not found
    FileNotFound = 2,
    /// File found, but could not be loaded
    FileLoadFailed = 3,
    /// DLL not found, or wrong DLL
    DllNotFound = 4,
    /// Out of memory
    OutOfMemory = 5,
    /// Feature not implemented
    NotImplemented = 6,
    /// Other error   
    UnknownError = 7,
}

impl SoloudErrorKind {
    /// Get a Soloud error from the returned i32 error code
    pub fn from_i32(val: i32) -> SoloudErrorKind {
        unsafe { std::mem::transmute(val) }
    }
}

impl std::error::Error for SoloudError {
    /// Get the source of the Soloud Error
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

bitflags::bitflags! {
    /// Soloud flags
    pub struct SoloudFlag: i32 {
        /// Clip roundoff
        const ClipRoundoff = 1;
        /// Enable visualization
        const EnableVisualization = 2;
        /// Left handed 3D
        const LeftHanded3D = 4;
        /// No Fpu register change
        const NoFpuRegisterChange = 8;
    }
}

/// Waveform types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum WaveForm {
    /// Square waveform
    Square = 0,
    /// Saw waveform
    Saw,
    /// Sin waveform
    Sin,
    /// Triangle waveform
    Triangle,
    /// Bounce waveform
    Bounce,
    /// Jaws waveform
    Jaws,
    /// Humps waveform
    Humps,
    /// Fsquare waveform
    FSquare,
    /// Fsaw waveform
    FSaw,
}

/// Resampler types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Resampler {
    /// Point resampler
    Point,
    /// Linear resampler
    Linear,
    /// Catmullrom resampler
    Catmullrom,
}

/// Attenuation models
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum AttenuationModel {
    /// No attenuation
    NoAttenuation = 0,
    /// Inverse distance attenuation model
    InverseDistance = 1,
    /// Linear distance attenuation model
    LinearDistance = 2,
    /// Exponential distance attenuation model
    ExponentialDistance = 3,
}

/// Methods shared by all audio sources
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

/// Methods for initializing sound sources from memory
pub unsafe trait LoadExt {
    /// Load audio from a file
    fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SoloudError>;
    /// Load audio from memory.
    fn load_mem(&mut self, data: &[u8]) -> Result<(), SoloudError> {
        unsafe { self._load_mem_ex(&data, false, false) }
    }
    #[doc(hidden)]
    /// (Internal) load audio from memory with options to copy and/or take ownership
    /// # Safety
    /// The audio source should not be invalidated
    unsafe fn _load_mem_ex(
        &mut self,
        data: &[u8],
        copy: bool,
        take_ownership: bool,
    ) -> Result<(), SoloudError>;
}

/// Filter creation and setting methods
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

/// A trait applicable to all filter attributes
pub trait FilterAttr {
    /// Convert a filter attribute to a u32
    fn to_u32(self) -> u32;
}

/// A trait defining methods of initializing audio sources from a path or memory
pub unsafe trait FromExt: Sized {
    /// Loads an audio source from path
    fn from_path<P: AsRef<Path>>(p: P) -> Result<Self, SoloudError>;
    /// Loads an audio source from memory. Prefer [`LoadExt::load_mem`] when possible like when
    /// using `'static &[u8]`
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
