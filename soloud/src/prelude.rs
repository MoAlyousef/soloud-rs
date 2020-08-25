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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum AttenuationModels {
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

pub unsafe trait AudioSource {
    fn set_volume(&mut self, aVolume: f32);

    fn set_looping(&mut self, aLoop: bool);

    fn set_auto_stop(&mut self, aAutoStop: bool);

    fn set_3d_min_max_distance(&mut self, aMinDistance: f32, aMaxDistance: f32);

    fn set_3d_attenuation(
        &mut self,
        aAttenuationModel: AttenuationModels,
        aAttenuationRolloffFactor: f32,
    );

    fn set_3d_doppler_factor(&mut self, aDopplerFactor: f32);

    fn set_3d_listener_relative(&mut self, aListenerRelative: bool);

    fn set_3d_distance_delay(&mut self, aDistanceDelay: i32);

    fn set_3d_collider(&mut self, aCollider: Option<&mut AudioCollider>);

    fn set_3d_attenuator(&mut self, aAttenuator: Option<&mut AudioAttenuator>);

    fn set_inaudible_behavior(&mut self, aMustTick: bool, aKill: bool);

    fn set_loop_point(&mut self, aLoopPoint: f64);

    fn get_loop_point(&mut self) -> f64;

    fn set_filter(&mut self, aFilterId: u32, aFilter: Option<&mut Filter>);

    fn stop(&mut self);

    fn inner(&self) -> *mut *mut std::os::raw::c_void;
}
