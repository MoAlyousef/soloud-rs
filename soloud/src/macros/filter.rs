macro_rules! impl_filter_ext {
    ($name: ident) => {
        paste::paste! {
            unsafe impl FilterExt for $name {
                unsafe fn inner(&self) -> *mut *mut std::os::raw::c_void {
                    self.inner
                }

                fn default() -> Self {
                    unsafe {
                        let ptr = soloud_sys::soloud::[<$name _create>]();
                        assert!(!ptr.is_null());
                        $name {
                            inner: ptr,
                        }
                    }
                }

                fn param_count(&mut self) -> i32 {

                    unsafe {
                        soloud_sys::soloud::[<$name _getParamCount>](self.inner)
                    }
                }


                fn param_name(&mut self, param_idx: u32) -> Option<String> {

                    unsafe {
                        let ptr = soloud_sys::soloud::[<$name _getParamName>](self.inner, param_idx);
                        if ptr.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string())
                        }
                    }
                }


                fn param_type(&mut self, param_idx: u32) -> ParamType {

                    unsafe {
                        std::mem::transmute(soloud_sys::soloud::[<$name _getParamType>](self.inner, param_idx))
                    }
                }


                fn param_max(&mut self, param_idx: u32) -> f32 {

                    unsafe {
                        soloud_sys::soloud::[<$name _getParamMax>](self.inner, param_idx)
                    }
                }


                fn param_min(&mut self, param_idx: u32) -> f32 {

                    unsafe {
                        soloud_sys::soloud::[<$name _getParamMin>](self.inner, param_idx)
                    }
                }
            }

            impl Drop for $name {
                fn drop(&mut self) {
                    unsafe { soloud_sys::soloud::[<$name _destroy>](self.inner) }
                }
            }
        }
    };
}

macro_rules! impl_filter_type {
    ($name: ident) => {
        impl FilterAttr for $name {
            fn to_u32(self) -> u32 {
                self as u32
            }
        }
    };
}

pub(crate) use impl_filter_ext;
pub(crate) use impl_filter_type;
