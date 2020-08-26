use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_filter_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = name.to_string();

    let destroy = Ident::new(format!("{}_{}", name_str, "destroy").as_str(), name.span());
    let create = Ident::new(format!("{}_{}", name_str, "create").as_str(), name.span());
    let getParamCount = Ident::new(format!("{}_{}", name_str, "getParamCount").as_str(), name.span());
    let getParamName = Ident::new(format!("{}_{}", name_str, "getParamName").as_str(), name.span());
    let getParamType = Ident::new(format!("{}_{}", name_str, "getParamType").as_str(), name.span());
    let getParamMax = Ident::new(format!("{}_{}", name_str, "getParamMax").as_str(), name.span());
    let getParamMin = Ident::new(format!("{}_{}", name_str, "getParamMin").as_str(), name.span());
    let setParams = Ident::new(format!("{}_{}", name_str, "setParams").as_str(), name.span());
    let setParamsEx = Ident::new(format!("{}_{}", name_str, "setParamsEx").as_str(), name.span());

    let gen = quote! {
        unsafe impl FilterExt for #name {
            fn inner(&self) -> *mut *mut std::os::raw::c_void {
                self._inner
            }

            fn default() -> Self {
                unsafe {
                    let ptr = soloud_sys::soloud::#create();
                    assert!(!ptr.is_null());
                    #name {
                        _inner: ptr,
                    }
                }
            }
            
            fn param_count(&mut self) -> i32 {
                unsafe {
                    soloud_sys::soloud::#getParamCount(self._inner)
                }
            }
        
        
            fn param_name( &mut self, aParamIndex: u32) -> Option<String> {
                unsafe {
                    let ptr = soloud_sys::soloud::#getParamName(self._inner, aParamIndex);
                    if ptr.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string())
                    }
                }
            }
        
        
            fn param_type( &mut self, aParamIndex: u32) -> ParamType {
                unsafe {
                    std::mem::transmute(soloud_sys::soloud::#getParamType(self._inner, aParamIndex))
                }
            }
        
        
            fn param_max(&mut self, aParamIndex: u32) -> f32 {
                unsafe {
                    soloud_sys::soloud::#getParamMax(self._inner, aParamIndex)
                }
            }
        
        
            fn param_min(&mut self, aParamIndex: u32) -> f32 {
                unsafe {
                    soloud_sys::soloud::#getParamMin(self._inner, aParamIndex)
                }
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                unsafe { soloud_sys::soloud::#destroy(self._inner) }
            }
        }
    };
    gen.into()
}

pub fn impl_filter_type_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl FilterType for #name {
            fn to_u32(self) -> u32 {
                self as u32
            }
        }
    };
    gen.into()
}