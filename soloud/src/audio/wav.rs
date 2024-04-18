use crate::prelude::*;
use soloud_sys::soloud as ffi;
use std::path::Path;

/// Wav audio type
#[derive(Debug)]
pub struct Wav {
    inner: *mut ffi::Wav,
}

crate::macros::load::impl_load_ext!(Wav);
crate::macros::audio::impl_audio_ext!(Wav);

impl Wav {
    /// Create immutable Wav instance with already loaded music
    ///
    /// # Examples
    ///
    /// use soloud::*;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let sl = Soloud::default()?;
    ///
    ///     let wav = audio::Wav::default_with_load("sample.wav")?;
    ///
    ///     sl.play(&wav);
    ///     while sl.voice_count() > 0 {
    ///         std::thread::sleep(std::time::Duration::from_millis(100));
    ///     }
    ///
    ///     Ok(())
    /// }
    /// 
    pub fn default_with_load<P: AsRef<Path>>(path: P) -> Result<Self, SoloudError> {
        let ptr = unsafe { ffi::Wav_create() };
        assert!(!ptr.is_null());
        let mut wav = Wav { inner: ptr };
        match wav.load(path) {
            Ok(_)    => Ok(wav),
            Err(err) => Err(err)
        }
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_8(&mut self, data: &[u8]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave8(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_8_ex(
        &mut self,
        data: &[u8],
        samplerate: f32,
        channels: u32,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave8Ex(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_16(&mut self, data: &[i16]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave16(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_16_ex(
        &mut self,
        data: &[i16],
        samplerate: f32,
        channels: u32,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave16Ex(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav(&mut self, data: &[f32]) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWave(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32
        ))
    }

    /// Load raw wav data of precise bits
    /// # Safety
    /// The data must be valid
    pub unsafe fn load_raw_wav_ex(
        &mut self,
        data: &[f32],
        samplerate: f32,
        channels: u32,
        copy: bool,
        take_ownership: bool,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Wav_loadRawWaveEx(
            self.inner,
            data.as_ptr() as *mut _,
            data.len() as u32,
            samplerate,
            channels,
            copy as i32,
            take_ownership as i32,
        ))
    }

    /// Get the length of the Wav object
    pub fn length(&self) -> f64 {
        unsafe { ffi::Wav_getLength(self.inner) }
    }
}
