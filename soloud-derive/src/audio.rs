use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_audio_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = name.to_string();

    let setVolume = Ident::new(
        format!("{}_{}", name_str, "setVolume").as_str(),
        name.span(),
    );
    let setLooping = Ident::new(
        format!("{}_{}", name_str, "setLooping").as_str(),
        name.span(),
    );
    let setAutoStop = Ident::new(
        format!("{}_{}", name_str, "setAutoStop").as_str(),
        name.span(),
    );
    let set3dMinMaxDistance = Ident::new(
        format!("{}_{}", name_str, "set3dMinMaxDistance").as_str(),
        name.span(),
    );
    let set3dAttenuation = Ident::new(
        format!("{}_{}", name_str, "set3dAttenuation").as_str(),
        name.span(),
    );
    let set3dDopplerFactor = Ident::new(
        format!("{}_{}", name_str, "set3dDopplerFactor").as_str(),
        name.span(),
    );
    let set3dListenerRelative = Ident::new(
        format!("{}_{}", name_str, "set3dListenerRelative").as_str(),
        name.span(),
    );
    let set3dDistanceDelay = Ident::new(
        format!("{}_{}", name_str, "set3dDistanceDelay").as_str(),
        name.span(),
    );
    // let set3dCollider = Ident::new(
    //     format!("{}_{}", name_str, "set3dCollider").as_str(),
    //     name.span(),
    // );
    // let set3dAttenuator = Ident::new(
    //     format!("{}_{}", name_str, "set3dAttenuator").as_str(),
    //     name.span(),
    // );
    let setInaudibleBehavior = Ident::new(
        format!("{}_{}", name_str, "setInaudibleBehavior").as_str(),
        name.span(),
    );
    let setLoopPoint = Ident::new(
        format!("{}_{}", name_str, "setLoopPoint").as_str(),
        name.span(),
    );
    let getLoopPoint = Ident::new(
        format!("{}_{}", name_str, "getLoopPoint").as_str(),
        name.span(),
    );
    let setFilter = Ident::new(
        format!("{}_{}", name_str, "setFilter").as_str(),
        name.span(),
    );
    let stop = Ident::new(format!("{}_{}", name_str, "stop").as_str(), name.span());
    let destroy = Ident::new(format!("{}_{}", name_str, "destroy").as_str(), name.span());
    let create = Ident::new(format!("{}_{}", name_str, "create").as_str(), name.span());

    let gen = quote! {

        unsafe impl AudioExt for #name {
            fn default() -> Self {
                let ptr = unsafe { ffi::#create() };
                assert!(!ptr.is_null());
                #name { _inner: ptr }
            }

            fn set_volume(&mut self, volume: f32) {

                unsafe {
                    soloud_sys::soloud::#setVolume(self._inner, volume)
                }
            }

            fn set_looping(&mut self, flag: bool) {

                unsafe {
                    soloud_sys::soloud::#setLooping(self._inner, flag as i32)
                }
            }

            fn set_auto_stop(&mut self, flag: bool) {

                unsafe {
                    soloud_sys::soloud::#setAutoStop(self._inner, flag as i32)
                }
            }

            fn set_3d_min_max_distance(&mut self, min_distance: f32, max_distance: f32) {

                unsafe {
                    soloud_sys::soloud::#set3dMinMaxDistance(self._inner, min_distance, max_distance)
                }
            }

            fn set_3d_attenuation(&mut self, model: AttenuationModel, rolloff_factor: f32) {

                unsafe {
                    soloud_sys::soloud::#set3dAttenuation(self._inner, model as u32, rolloff_factor)
                }
            }

            fn set_3d_doppler_factor(&mut self, doppler_factor: f32) {

                unsafe {
                    soloud_sys::soloud::#set3dDopplerFactor(self._inner, doppler_factor)
                }
            }

            fn set_3d_listener_relative(&mut self, flag: bool) {

                unsafe {
                    soloud_sys::soloud::#set3dListenerRelative(self._inner, flag as i32)
                }
            }

            fn set_3d_distance_delay(&mut self, distance_delay: i32) {

                unsafe {
                    soloud_sys::soloud::#set3dDistanceDelay(self._inner, distance_delay)
                }
            }

            // fn set_3d_collider(&mut self, collider: Option<&AudioCollider>) {
            //     let mut collider = match collider {
            //         Some(v) => unsafe { v.inner() },
            //         None => std::ptr::null_mut(),
            //     };

            //     unsafe {
            //         soloud_sys::soloud::#set3dCollider(self._inner, &mut collider as *mut *mut _)
            //     }
            // }

            // fn set_3d_attenuator(&mut self, attenuator: Option<&AudioAttenuator>) {
            //     let mut attenuator = match attenuator {
            //         Some(v) => unsafe { v.inner() },
            //         None => std::ptr::null_mut(),
            //     };

            //     unsafe {
            //         soloud_sys::soloud::#set3dAttenuator(self._inner, &mut attenuator as *mut *mut _)
            //     }
            // }

            fn set_inaudible_behavior(&mut self, must_tick: bool, kill: bool) {

                unsafe {
                    soloud_sys::soloud::#setInaudibleBehavior(self._inner, must_tick as i32, kill as i32)
                }
            }

            fn set_loop_point(&mut self, loop_point: f64) {

                unsafe {
                    soloud_sys::soloud::#setLoopPoint(self._inner, loop_point)
                }
            }

            fn loop_point(&self) -> f64 {

                unsafe {
                    soloud_sys::soloud::#getLoopPoint(self._inner)
                }
            }

            fn set_filter<F: FilterExt>(&mut self, filter_id: u32, filter: Option<&F>) {
                let filter = match filter {
                    Some(v) => unsafe { v.inner() },
                    None => std::ptr::null_mut(),
                };

                unsafe {
                    soloud_sys::soloud::#setFilter(self._inner, filter_id, filter)
                }
            }

            fn stop(&mut self) {

                unsafe {
                    soloud_sys::soloud::#stop(self._inner)
                }
            }

            unsafe fn inner(&self) -> *mut *mut std::os::raw::c_void {
                self._inner
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe {
                    soloud_sys::soloud::#destroy(self._inner);
                    self._inner = std::ptr::null_mut();
                }
            }
        }
    };
    gen.into()
}
