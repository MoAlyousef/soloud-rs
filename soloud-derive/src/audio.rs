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
    let set3dCollider = Ident::new(
        format!("{}_{}", name_str, "set3dCollider").as_str(),
        name.span(),
    );
    let set3dAttenuator = Ident::new(
        format!("{}_{}", name_str, "set3dAttenuator").as_str(),
        name.span(),
    );
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

            fn set_volume(&mut self, aVolume: f32) {
                unsafe {
                    soloud_sys::soloud::#setVolume(self._inner, aVolume)
                }
            }

            fn set_looping(&mut self, aLoop: bool) {
                unsafe {
                    soloud_sys::soloud::#setLooping(self._inner, aLoop as i32)
                }
            }

            fn set_auto_stop(&mut self, aAutoStop: bool) {
                unsafe {
                    soloud_sys::soloud::#setAutoStop(self._inner, aAutoStop as i32)
                }
            }

            fn set_3d_min_max_distance(&mut self, aMinDistance: f32, aMaxDistance: f32) {
                unsafe {
                    soloud_sys::soloud::#set3dMinMaxDistance(self._inner, aMinDistance, aMaxDistance)
                }
            }

            fn set_3d_attenuation(&mut self,aAttenuationModel: AttenuationModels,aAttenuationRolloffFactor: f32) {
                unsafe {
                    soloud_sys::soloud::#set3dAttenuation(self._inner, aAttenuationModel as u32, aAttenuationRolloffFactor)
                }
            }

            fn set_3d_doppler_factor(&mut self, aDopplerFactor: f32) {
                unsafe {
                    soloud_sys::soloud::#set3dDopplerFactor(self._inner, aDopplerFactor)
                }
            }

            fn set_3d_listener_relative(&mut self, aListenerRelative: bool) {
                unsafe {
                    soloud_sys::soloud::#set3dListenerRelative(self._inner, aListenerRelative as i32)
                }
            }

            fn set_3d_distance_delay(&mut self, aDistanceDelay: i32) {
                unsafe {
                    soloud_sys::soloud::#set3dDistanceDelay(self._inner, aDistanceDelay)
                }
            }

            fn set_3d_collider(&mut self, aCollider: Option<&AudioCollider>) {
                let aCollider = match aCollider {
                    Some(v) => v.inner(),
                    None => std::ptr::null_mut(),
                };
                unsafe {
                    soloud_sys::soloud::#set3dCollider(self._inner, aCollider)
                }
            }

            fn set_3d_attenuator(&mut self, aAttenuator: Option<&AudioAttenuator>) {
                let aAttenuator = match aAttenuator {
                    Some(v) => v.inner(),
                    None => std::ptr::null_mut(),
                };
                unsafe {
                    soloud_sys::soloud::#set3dAttenuator(self._inner, aAttenuator)
                }
            }

            fn set_inaudible_behavior(&mut self,aMustTick: bool,aKill: bool) {
                unsafe {
                    soloud_sys::soloud::#setInaudibleBehavior(self._inner,aMustTick as i32,aKill as i32)
                }
            }

            fn set_loop_point(&mut self, aLoopPoint: f64) {
                unsafe {
                    soloud_sys::soloud::#setLoopPoint(self._inner, aLoopPoint)
                }
            }

            fn loop_point(&self) -> f64 {
                unsafe {
                    soloud_sys::soloud::#getLoopPoint(self._inner)
                }
            }

            fn set_filter<F: FilterExt>(&mut self, aFilterId: u32, aFilter: Option<&F>) {
                let aFilter = match aFilter {
                    Some(v) => v.inner(),
                    None => std::ptr::null_mut(),
                };
                unsafe {
                    soloud_sys::soloud::#setFilter(self._inner, aFilterId, aFilter)
                }
            }

            fn stop(&mut self) {
                unsafe {
                    soloud_sys::soloud::#stop(self._inner)
                }
            }

            fn inner(&self) -> *mut *mut std::os::raw::c_void {
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
