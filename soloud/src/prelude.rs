use std::convert::From;
use std::os::raw::*;
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
pub enum SoloudFlag {
    ClipRoundoff = 1,
    EnableVisualization = 2,
    LeftHanded3D = 4,
    NoFpuRegisterChange = 8,
}

impl std::ops::BitOr for SoloudFlag {
    type Output = Self;

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

pub struct AudioSourceInstance3dData {
    _inner: *mut c_void,
}

impl AudioSourceInstance3dData {
    /// Instantiante a new AudioSourceInstance3dData
    pub fn new(engine: &crate::Soloud) -> Self {
        unsafe {
            let ptr = soloud_sys::soloud_derives::AudioSourceInstance3dData_new(
                engine.inner() as *mut c_void
            );
            assert!(!ptr.is_null());
            AudioSourceInstance3dData { _inner: ptr }
        }
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        assert!(!ptr.is_null());
        AudioSourceInstance3dData { _inner: ptr }
    }
}

impl Drop for AudioSourceInstance3dData {
    fn drop(&mut self) {
        unsafe { soloud_sys::soloud_derives::AudioSourceInstance3dData_delete(self._inner) }
    }
}

/// Audio Collider struct
pub struct AudioCollider {
    _inner: *mut soloud_sys::soloud::AudioCollider,
}

impl AudioCollider {
    /// Instantiate a new AudioCollider
    pub fn default() -> Self {
        unsafe {
            let ptr = soloud_sys::soloud_derives::AudioCollider_new();
            assert!(!ptr.is_null());
            AudioCollider {
                _inner: ptr as *mut soloud_sys::soloud::AudioCollider,
            }
        }
    }

    /// Override the collide method
    pub unsafe fn collide(
        &mut self,
        cb: Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32>,
    ) {
        unsafe {
            unsafe extern "C" fn shim(
                arg1: *mut c_void,
                arg2: *mut c_void,
                arg3: c_int,
                data: *mut c_void,
            ) -> f32 {
                let a: *mut Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32> =
                    data as *mut Box<
                        dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32,
                    >;
                let f: &mut (dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32) =
                    &mut **a;
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        &crate::Soloud::from_ptr(arg1),
                        &AudioSourceInstance3dData::from_ptr(arg2),
                        arg3,
                    )
                }))
                .unwrap_or(0.0)
            }
            let a: *mut Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32> =
                Box::into_raw(Box::new(cb));
            let data: *mut c_void = a as *mut c_void;
            let callback: Option<
                unsafe extern "C" fn(
                    arg1: *mut c_void,
                    arg2: *mut c_void,
                    arg3: c_int,
                    arg4: *mut c_void,
                ) -> f32,
            > = Some(shim);
            soloud_sys::soloud_derives::AudioCollider_set_handler(
                self._inner as *mut c_void,
                callback,
                data,
            );
        }
    }

    /// pub unsafe fn inner(&self) -> *mut soloud_sys::soloud::AudioCollider
    /// # Safety
    /// The inner pointer should be modified with care!
    pub unsafe fn inner(&self) -> *mut soloud_sys::soloud::AudioCollider {
        self._inner
    }
}

impl Drop for AudioCollider {
    fn drop(&mut self) {
        unsafe { soloud_sys::soloud_derives::AudioCollider_delete(self._inner as *mut c_void) }
    }
}

/// Audio Attenuator struct
pub struct AudioAttenuator {
    _inner: *mut soloud_sys::soloud::AudioAttenuator,
}

impl AudioAttenuator {
    /// Instantiate a new AudioAttenuator
    pub fn default() -> Self {
        unsafe {
            let ptr = soloud_sys::soloud_derives::AudioAttenuator_new();
            assert!(!ptr.is_null());
            AudioAttenuator {
                _inner: ptr as *mut soloud_sys::soloud::AudioAttenuator,
            }
        }
    }
    
    /// Override the attenuate method
    pub fn attenuate(&mut self, cb: Box<dyn FnMut(f32, f32, f32, f32) -> f32>) {
        unsafe {
            unsafe extern "C" fn shim(
                arg1: f32,
                arg2: f32,
                arg3: f32,
                arg4: f32,
                data: *mut c_void,
            ) -> f32 {
                let a: *mut Box<dyn FnMut(f32, f32, f32, f32) -> f32> =
                    data as *mut Box<dyn FnMut(f32, f32, f32, f32) -> f32>;
                let f: &mut (dyn FnMut(f32, f32, f32, f32) -> f32) = &mut **a;
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(arg1, arg2, arg3, arg4)))
                    .unwrap_or(0.0)
            }
            let a: *mut Box<dyn FnMut(f32, f32, f32, f32) -> f32> = Box::into_raw(Box::new(cb));
            let data: *mut c_void = a as *mut c_void;
            let callback: Option<
                unsafe extern "C" fn(
                    arg1: f32,
                    arg2: f32,
                    arg3: f32,
                    arg4: f32,
                    data: *mut c_void,
                ) -> f32,
            > = Some(shim);
            soloud_sys::soloud_derives::AudioAttenuator_set_handler(
                self._inner as *mut c_void,
                callback,
                data,
            );
        }
    }

    /// pub unsafe fn inner(&self) -> *mut soloud_sys::soloud::AudioAttenuator
    /// # Safety
    /// The inner pointer should be modified with care!
    pub unsafe fn inner(&self) -> *mut soloud_sys::soloud::AudioAttenuator {
        self._inner
    }
}

impl Drop for AudioAttenuator {
    fn drop(&mut self) {
        unsafe { soloud_sys::soloud_derives::AudioAttenuator_delete(self._inner as *mut c_void) }
    }
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
    /// Set 3D collider
    fn set_3d_collider(&mut self, collider: Option<&AudioCollider>);
    /// Set 3D attenuator
    fn set_3d_attenuator(&mut self, attenuator: Option<&AudioAttenuator>);
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
    fn load(&mut self, path: &std::path::Path) -> Result<(), SoloudError>;
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
