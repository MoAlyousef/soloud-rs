use crate::{prelude::*, Handle};
use soloud_sys::soloud as ffi;

/// Bus audio type
#[derive(Debug)]
pub struct Bus {
    inner: *mut ffi::Bus,
}

crate::macros::audio::impl_audio_ext!(Bus);

impl Bus {
    /// Get active voice count
    pub fn active_voice_count(&self) -> u32 {
        unsafe { ffi::Bus_getActiveVoiceCount(self.inner) }
    }

    /// Play audio
    pub fn play<T: AudioExt>(&self, sound: &T) -> Handle {
        Handle(unsafe { ffi::Bus_play(self.inner, sound.inner()) })
    }

    /// Play audio with extra args
    pub fn play_ex<AS: AudioExt>(&self, sound: &AS, volume: f32, pan: f32, paused: bool) -> Handle {
        Handle(unsafe { ffi::Bus_playEx(self.inner, sound.inner(), volume, pan, paused as i32) })
    }

    /// Play clocked
    pub fn play_clocked<AS: AudioExt>(&self, sound_time: f64, sound: &AS) -> Handle {
        Handle(unsafe { ffi::Bus_playClocked(self.inner, sound_time, sound.inner()) })
    }

    /// Play clocked with extra args
    pub fn play_clocked_ex<AS: AudioExt>(
        &self,
        sound_time: f64,
        sound: &AS,
        volume: f32,
        pan: f32,
    ) -> Handle {
        Handle(unsafe {
            ffi::Bus_playClockedEx(self.inner, sound_time, sound.inner(), volume, pan)
        })
    }

    /// Play 3D
    pub fn play_3d<AS: AudioExt>(&self, sound: &AS, pos_x: f32, pos_y: f32, pos_z: f32) -> Handle {
        Handle(unsafe { ffi::Bus_play3d(self.inner, sound.inner(), pos_x, pos_y, pos_z) })
    }

    /// Play 3D with extra args
    pub fn play_3d_ex<AS: AudioExt>(
        &self,
        sound: &AS,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        vel_x: f32,
        vel_y: f32,
        vel_z: f32,
        volume: f32,
        paused: bool,
    ) -> Handle {
        Handle(unsafe {
            ffi::Bus_play3dEx(
                self.inner,
                sound.inner(),
                pos_x,
                pos_y,
                pos_z,
                vel_x,
                vel_y,
                vel_z,
                volume,
                paused as i32,
            )
        })
    }

    /// Play 3D clocked
    pub fn play_3d_clocked<AS: AudioExt>(
        &self,
        sound_time: f64,
        sound: &AS,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
    ) -> Handle {
        Handle(unsafe {
            ffi::Bus_play3dClocked(self.inner, sound_time, sound.inner(), pos_x, pos_y, pos_z)
        })
    }

    /// Play 3D clocked with extra args
    pub fn play_3d_clocked_ex<AS: AudioExt>(
        &self,
        sound_time: f64,
        sound: &AS,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        vel_x: f32,
        vel_y: f32,
        vel_z: f32,
        volume: f32,
    ) -> Handle {
        Handle(unsafe {
            ffi::Bus_play3dClockedEx(
                self.inner,
                sound_time,
                sound.inner(),
                pos_x,
                pos_y,
                pos_z,
                vel_x,
                vel_y,
                vel_z,
                volume,
            )
        })
    }

    /// Enable visualizations
    pub fn set_visualize_enable(&self, flag: bool) {
        unsafe { ffi::Bus_setVisualizationEnable(self.inner, flag as i32) }
    }

    /// Calculate and get 256 floats of FFT data for visualization. Visualization has to be enabled before use
    pub fn calc_fft(&self) -> Vec<f32> {
        unsafe {
            let ret = ffi::Bus_calcFFT(self.inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }

    /// Get 256 floats of wave data for visualization. Visualization has to be enabled before use
    pub fn wave(&self) -> Vec<f32> {
        unsafe {
            let ret = ffi::Bus_getWave(self.inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }

    /// Get approximate volume
    pub fn approximate_volume(&self, channel: u32) -> f32 {
        unsafe { ffi::Bus_getApproximateVolume(self.inner, channel) }
    }

    /// Set bus channels
    pub fn set_channels(&mut self, channels: u32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Bus_setChannels(self.inner, channels);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    /// Get bus resampler
    pub fn resampler(&self) -> Resampler {
        unsafe { std::mem::transmute(ffi::Bus_getResampler(self.inner)) }
    }

    /// Set bus resampler
    pub fn set_resampler(&mut self, resampler: Resampler) {
        unsafe { ffi::Bus_setResampler(self.inner, resampler as u32) }
    }
}
