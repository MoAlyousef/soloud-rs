use soloud_sys::soloud as ffi;
use std::os::raw::*;

pub struct AudioSourceInstance3dData {
    _inner: *mut ffi::AudioSourceInstance3dData,
}

impl AudioSourceInstance3dData {
    /// Instantiante a new AudioSourceInstance3dData
    pub fn new(engine: &crate::Soloud) -> Self {
        unsafe {
            let ptr = ffi::AudioSourceInstance3dData_new(
                engine.inner() as *mut c_void
            );
            assert!(!ptr.is_null());
            AudioSourceInstance3dData { _inner: ptr }
        }
    }

    unsafe fn from_ptr(ptr: *mut *mut c_void) -> Self {
        assert!(!ptr.is_null());
        AudioSourceInstance3dData { _inner: ptr }
    }
}

impl Drop for AudioSourceInstance3dData {
    fn drop(&mut self) {
        unsafe { ffi::AudioSourceInstance3dData_delete(self._inner) }
    }
}

/// Audio Collider struct
pub struct AudioCollider {
    _inner: *mut ffi::AudioCollider,
}

impl AudioCollider {
    /// Instantiate a new AudioCollider
    pub fn default() -> Self {
        unsafe {
            let ptr = ffi::AudioCollider_new();
            assert!(!ptr.is_null());
            AudioCollider { _inner: ptr }
        }
    }

    /// Override the collide method
    /// collide(aSoloud, aAudioInstance3dData, aUserData)
    pub fn collide(
        &mut self,
        cb: Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32>,
    ) {
        unsafe {
            unsafe extern "C" fn shim(
                arg1: *mut *mut c_void,
                arg2: *mut *mut c_void,
                arg3: c_int,
                data: *mut c_void,
            ) -> f32 {
                let a: *mut Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32> =
                    data as *mut Box<
                        dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32,
                    >;
                let f: &mut (dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32) =
                    &mut **a;
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        &crate::Soloud::from_ptr(*arg1),
                        &AudioSourceInstance3dData::from_ptr(arg2),
                        arg3,
                    )
                }));
                if let Ok(res) = res {
                    res
                } else {
                    0.0
                }
            }
            let a: *mut Box<dyn FnMut(&crate::Soloud, &AudioSourceInstance3dData, i32) -> f32> =
                Box::into_raw(Box::new(cb));
            let data: *mut c_void = a as *mut c_void;
            let callback: Option<
                unsafe extern "C" fn(
                    arg1: *mut *mut c_void,
                    arg2: *mut *mut c_void,
                    arg3: c_int,
                    arg4: *mut c_void,
                ) -> f32,
            > = Some(shim);
            ffi::AudioCollider_set_handler(
                self._inner as *mut c_void,
                callback,
                data,
            );
        }
    }

    /// Gets the inner ptr of the audio collider
    /// # Safety
    /// The inner pointer should be modified with care!
    pub unsafe fn inner(&self) -> *mut ffi::AudioCollider {
        self._inner
    }
}

impl Drop for AudioCollider {
    fn drop(&mut self) {
        unsafe { ffi::AudioCollider_delete(self._inner) }
    }
}

/// Audio Attenuator struct
pub struct AudioAttenuator {
    _inner: *mut ffi::AudioAttenuator,
}

impl AudioAttenuator {
    /// Instantiate a new AudioAttenuator
    pub fn default() -> Self {
        unsafe {
            let ptr = ffi::AudioAttenuator_new();
            assert!(!ptr.is_null());
            AudioAttenuator { _inner: ptr }
        }
    }

    /// Override the attenuate method
    /// attenuate(aDistance, aMinDistance, aMaxDistance, aRolloffFactor)
    pub fn attenuate(&mut self, cb: Box<dyn FnMut(f32, f32, f32, f32) -> f32>) {
        assert!(!self._inner.is_null());
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
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(arg1, arg2, arg3, arg4)
                }));
                if let Ok(res) = res {
                    res
                } else {
                    0.0
                }
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
            ffi::AudioAttenuator_set_handler(
                self._inner as *mut c_void,
                callback,
                data,
            );
        }
    }

    /// Gets the inner ptr of the audio attenuator
    /// # Safety
    /// The inner pointer should be modified with care!
    pub unsafe fn inner(&self) -> *mut ffi::AudioAttenuator {
        self._inner
    }
}

impl Drop for AudioAttenuator {
    fn drop(&mut self) {
        unsafe { ffi::AudioAttenuator_delete(self._inner) }
    }
}