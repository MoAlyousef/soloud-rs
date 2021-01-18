use crate::prelude::*;
use soloud_sys::soloud as ffi;

/// Queue audio type
#[derive(Debug, AudioExt)]
pub struct Queue {
    _inner: *mut ffi::Queue,
}

impl Queue {
    /// Play audio
    pub fn play<T: AudioExt>(&self, sound: &T) -> Result<(), SoloudError> {
        ffi_call!(ffi::Queue_play(self._inner, sound.inner()))
    }

    /// Get queue count
    pub fn get_count(&self) -> u32 {
        unsafe { ffi::Queue_getQueueCount(self._inner) }
    }

    /// Check if audio source is currently playing
    pub fn is_currently_playing<AS: AudioExt>(&self, sound: &AS) -> bool {
        unsafe { ffi::Queue_isCurrentlyPlaying(self._inner, sound.inner()) != 0 }
    }

    /// Set params from audio source
    pub fn set_params_from_audio_source<AS: AudioExt>(
        &mut self,
        sound: &AS,
    ) -> Result<(), SoloudError> {
        ffi_call!(ffi::Queue_setParamsFromAudioSource(
            self._inner,
            sound.inner()
        ))
    }

    /// Set params of the queue
    pub fn set_params(&mut self, samplerate: f32) -> Result<(), SoloudError> {
        ffi_call!(ffi::Queue_setParams(self._inner, samplerate))
    }

    /// Set params of the queue adding channels
    pub fn set_params_ex(&mut self, samplerate: f32, channels: u32) -> Result<(), SoloudError> {
        ffi_call!(ffi::Queue_setParamsEx(self._inner, samplerate, channels))
    }
}
