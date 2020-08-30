use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_load_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let name_str = name.to_string();

    let load = Ident::new(format!("{}_{}", name_str, "load").as_str(), name.span());
    let loadMem = Ident::new(format!("{}_{}", name_str, "loadMem").as_str(), name.span());
    let loadMemEx = Ident::new(
        format!("{}_{}", name_str, "loadMemEx").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl LoadExt for #name {
            fn load(&mut self, path: &std::path::Path) -> Result<(), SoloudError> {
                
                unsafe {
                    let path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
                    let ret = soloud_sys::soloud::#load(self._inner, path.as_ptr());
                    if ret != 0 {
                        Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
                    } else {
                        Ok(())
                    }
                }
            }

            fn load_mem(&mut self, data: &[u8]) -> Result<(), SoloudError> {
                
                unsafe {
                    let ret = soloud_sys::soloud::#loadMemEx(self._inner, data.as_ptr(), data.len() as u32, 0, 0);
                    if ret != 0 {
                        Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
                    } else {
                        Ok(())
                    }
                }
            }

            unsafe fn load_mem_ex(&mut self, data: &[u8], copy: bool, take_ownership: bool) -> Result<(), SoloudError> {
                
                unsafe {
                    let ret = soloud_sys::soloud::#loadMemEx(self._inner, data.as_ptr(), data.len() as u32, copy as i32, take_ownership as i32);
                    if ret != 0 {
                        Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
                    } else {
                        Ok(())
                    }
                }
            }
        }
    };
    gen.into()
}
