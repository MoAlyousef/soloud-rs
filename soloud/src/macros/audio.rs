macro_rules! impl_audio_ext {
    ($name: ident) => {
        paste::paste! {
            unsafe impl AudioExt for $name {
                fn default() -> Self {
                    let ptr = unsafe { ffi::[<$name _create>]() };
                    assert!(!ptr.is_null());
                    $name { inner: ptr }
                }
            
                fn set_volume(&mut self, volume: f32) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setVolume>](self.inner, volume)
                    }
                }
            
                fn set_looping(&mut self, flag: bool) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setLooping>](self.inner, flag as i32)
                    }
                }
            
                fn set_auto_stop(&mut self, flag: bool) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setAutoStop>](self.inner, flag as i32)
                    }
                }
            
                fn set_3d_min_max_distance(&mut self, min_distance: f32, max_distance: f32) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _set3dMinMaxDistance>](self.inner, min_distance, max_distance)
                    }
                }
            
                fn set_3d_attenuation(&mut self, model: AttenuationModel, rolloff_factor: f32) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _set3dAttenuation>](self.inner, model as u32, rolloff_factor)
                    }
                }
            
                fn set_3d_doppler_factor(&mut self, doppler_factor: f32) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _set3dDopplerFactor>](self.inner, doppler_factor)
                    }
                }
            
                fn set_3d_listener_relative(&mut self, flag: bool) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _set3dListenerRelative>](self.inner, flag as i32)
                    }
                }
            
                fn set_3d_distance_delay(&mut self, distance_delay: i32) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _set3dDistanceDelay>](self.inner, distance_delay)
                    }
                }
            
                // fn set_3d_collider(&mut self, collider: Option<&AudioCollider>) {
                //     let mut collider = match collider {
                //         Some(v) => unsafe { v.inner() },
                //         None => std::ptr::null_mut(),
                //     };
            
                //     unsafe {
                //         soloud_sys::soloud::[<$name _set3dCollider>](self.inner, &mut collider as *mut *mut _)
                //     }
                // }
            
                // fn set_3d_attenuator(&mut self, attenuator: Option<&AudioAttenuator>) {
                //     let mut attenuator = match attenuator {
                //         Some(v) => unsafe { v.inner() },
                //         None => std::ptr::null_mut(),
                //     };
            
                //     unsafe {
                //         soloud_sys::soloud::[<$name _set3dAttenuator>](self.inner, &mut attenuator as *mut *mut _)
                //     }
                // }
            
                fn set_inaudible_behavior(&mut self, must_tick: bool, kill: bool) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setInaudibleBehavior>](self.inner, must_tick as i32, kill as i32)
                    }
                }
            
                fn set_loop_point(&mut self, loop_point: f64) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setLoopPoint>](self.inner, loop_point)
                    }
                }
            
                fn loop_point(&self) -> f64 {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _getLoopPoint>](self.inner)
                    }
                }
            
                fn set_filter<F: FilterExt>(&mut self, filter_id: u32, filter: Option<&F>) {
                    let filter = match filter {
                        Some(v) => unsafe { v.inner() },
                        None => std::ptr::null_mut(),
                    };
            
                    unsafe {
                        soloud_sys::soloud::[<$name _setFilter>](self.inner, filter_id, filter)
                    }
                }
            
                fn stop(&mut self) {
            
                    unsafe {
                        soloud_sys::soloud::[<$name _stop>](self.inner)
                    }
                }
            
                unsafe fn inner(&self) -> *mut *mut std::os::raw::c_void {
                    self.inner
                }
            }
            
            impl Drop for $name {
                fn drop(&mut self) {
                    unsafe {
                        soloud_sys::soloud::[<$name _destroy>](self.inner);
                        self.inner = std::ptr::null_mut();
                    }
                }
            }
        }
    };
}

pub(crate) use impl_audio_ext;