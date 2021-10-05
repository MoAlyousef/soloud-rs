macro_rules! impl_load_ext {
    ($name: ident) => {
        paste::paste! {
            unsafe impl LoadExt for $name {
                fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), SoloudError> {
                    unsafe {
                        let path = std::ffi::CString::new(path.as_ref().to_str().ok_or(SoloudError::Internal(SoloudErrorKind::FileLoadFailed))?)?;
                        let ret = soloud_sys::soloud::[<$name _load>](self.inner, path.as_ref().as_ptr());
                        if ret != 0 {
                            Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
                        } else {
                            Ok(())
                        }
                    }
                }
    
                unsafe fn _load_mem_ex(&mut self, data: &[u8], copy: bool, take_ownership: bool) -> Result<(), SoloudError> {
                    unsafe {
                        let ret = soloud_sys::soloud::[<$name _loadMemEx>](self.inner, data.as_ptr(), data.len() as u32, copy as i32, take_ownership as i32);
                        if ret != 0 {
                            Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
                        } else {
                            Ok(())
                        }
                    }
                }
            }
        }
    };
}

pub(crate) use impl_load_ext;