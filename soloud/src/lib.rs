#![allow(non_snake_case)]

pub mod prelude;
pub mod audio;
pub mod filter;

#[macro_use]
extern crate soloud_derive;

pub use prelude::*;
pub use audio::*;
pub use filter::*;

use soloud_sys::soloud as ffi;

pub type Handle = u32;

#[derive(Clone)]
pub struct Soloud {
    _inner: *mut ffi::Soloud,
}

unsafe impl Send for Soloud {}

unsafe impl Sync for Soloud {}

impl Soloud {
    pub fn default_uninit() -> std::mem::MaybeUninit<Self> {
        unsafe {
            let ptr = ffi::Soloud_create();
            assert!(!ptr.is_null());
            std::mem::MaybeUninit::new(Soloud { _inner: ptr })
        }
    }

    pub fn init(&mut self) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_init(self._inner);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn default() -> Result<Self, SoloudError> {
        let mut temp = unsafe { Soloud::default_uninit().assume_init() };
        if let Err(val) = temp.init() {
            Err(val)
        } else {
            Ok(temp)
        }
    }

    pub fn init_ex(&mut self, aFlags: SoloudFlags, aSamplerate: u32, aBufferSize: u32, aChannels: u32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_initEx(self._inner, aFlags as u32, 0, aSamplerate, aBufferSize, aChannels);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn version(&self) -> u32 {
        unsafe {
            ffi::Soloud_getVersion(self._inner)
        }
    }

    pub fn backend_id(&self) -> u32 {
        14
    }

    pub fn backend_string(&self) -> String {
        String::from("MINIAUDIO")
    }

    pub fn backend_channels(&self) -> u32 {
        unsafe {
            ffi::Soloud_getBackendChannels(self._inner)
        }
    }

    pub fn backend_samplerate(&self) -> u32 {
        unsafe {
            ffi::Soloud_getBackendSamplerate(self._inner)
        }
    }

    pub fn backend_buffer_size(&self) -> u32 {
        unsafe {
            ffi::Soloud_getBackendBufferSize(self._inner)
        }
    }

    pub fn set_speaker_position(&mut self, aChannel: u32, aX: f32, aY: f32, aZ: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_setSpeakerPosition(self._inner, aChannel, aX, aY, aZ);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn speaker_position(&self, aChannel: u32) -> Result<(f32, f32, f32), SoloudError> {
        unsafe {
            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;
            let ret = ffi::Soloud_getSpeakerPosition(self._inner, aChannel, &mut x, &mut y, &mut z);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok((x, y, z))
            }
        }
    }

    pub fn play_ex<AS: AudioExt>(&self, aSound: &AS, aVolume: f32, aPan: f32, aPaused: bool, aBus: u32) -> u32 {
        unsafe {
            ffi::Soloud_playEx(self._inner, aSound.inner(), aVolume, aPan, aPaused as i32, aBus)
        }
    }
    
    pub fn play_clocked<AS: AudioExt>(&self, aSoundTime: f64, aSound: &AS) -> u32 {
        unsafe {
            ffi::Soloud_playClocked(self._inner, aSoundTime, aSound.inner())
        }
    }
    
    pub fn play_clocked_ex<AS: AudioExt>(&self, aSoundTime: f64, aSound: &AS, aVolume: f32, aPan: f32, aBus: u32) -> u32 {
        unsafe {
            ffi::Soloud_playClockedEx(self._inner, aSoundTime, aSound.inner(), aVolume, aPan, aBus)
        }
    }
    
    pub fn play_3d<AS: AudioExt>(&self, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32) -> u32 {
        unsafe {
            ffi::Soloud_play3d(self._inner, aSound.inner(), aPosX, aPosY, aPosZ)
        }
    }
    
    pub fn play_3d_ex<AS: AudioExt>(&self, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32, aVelX: f32, aVelY: f32, aVelZ: f32, aVolume: f32, aPaused: bool, aBus: u32) -> u32 {
        unsafe {
            ffi::Soloud_play3dEx(self._inner, aSound.inner(), aPosX, aPosY, aPosZ, aVelX, aVelY, aVelZ, aVolume, aPaused as i32, aBus)
        }
    }
    
    pub fn play_3d_clocked<AS: AudioExt>(&self, aSoundTime: f64, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32) -> u32 {
        unsafe {
            ffi::Soloud_play3dClocked(self._inner, aSoundTime, aSound.inner(), aPosX, aPosY, aPosZ)
        }
    }
    
    pub fn play_3d_clocked_ex<AS: AudioExt>(&self, aSoundTime: f64, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32, aVelX: f32, aVelY: f32, aVelZ: f32, aVolume: f32, aBus: u32) -> u32 {
        unsafe {
            ffi::Soloud_play3dClockedEx(self._inner, aSoundTime, aSound.inner(), aPosX, aPosY, aPosZ, aVelX, aVelY, aVelZ, aVolume, aBus)
        }
    }
    
    pub fn play_background<AS: AudioExt>(&self, aSound: &AS) -> u32 {
        unsafe {
            ffi::Soloud_playBackground(self._inner, aSound.inner())
        }
    }
    
    pub fn play_background_ex<AS: AudioExt>(&self, aSound: &AS, aVolume: f32, aPaused: bool, aBus: u32) -> u32 {
        unsafe {
            ffi::Soloud_playBackgroundEx(self._inner, aSound.inner(), aVolume, aPaused as i32, aBus)
        }
    }

    pub fn seek(&self, aVoiceHandle: Handle, aSeconds: f64) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_seek(self._inner, aVoiceHandle, aSeconds);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }
    
    pub fn stop(&self, aVoiceHandle: Handle) {
        unsafe {
            ffi::Soloud_stop(self._inner, aVoiceHandle)
        }
    }
    
    pub fn stopAll(&self) {
        unsafe {
            ffi::Soloud_stopAll(self._inner)
        }
    }

    pub fn deinit(&mut self) {
        unsafe {
            ffi::Soloud_deinit(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }

    pub fn play<T: AudioExt>(&self, sound: &T) -> Handle {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_play(self._inner, sound.inner()) }
    }

    pub fn active_voice_count(&self) -> u32 {
        unsafe { ffi::Soloud_getActiveVoiceCount(self._inner) }
    }

    pub fn voice_count(&self) -> u32 {
        unsafe { ffi::Soloud_getVoiceCount(self._inner) }
    }

    pub fn set_global_volume(&mut self, val: f32) {
        unsafe { ffi::Soloud_setGlobalVolume(self._inner, val) }
    }

    pub fn stop_audio_source<AS: AudioExt>(&self, aSound: &AS) {
        unsafe {
            ffi::Soloud_stopAudioSource(self._inner, aSound.inner())
        }
    }

    pub fn count_audio_source<AS: AudioExt>(&mut self, aSound: &AS) -> i32 {
        unsafe {
            ffi::Soloud_countAudioSource(self._inner, aSound.inner())
        }
    }

    pub fn stream_time(&mut self, aVoiceHandle: Handle) -> f64 {
        unsafe {
            ffi::Soloud_getStreamTime(self._inner, aVoiceHandle)
        }
    }

    pub fn stream_position(&mut self, aVoiceHandle: Handle) -> f64 {
        unsafe {
            ffi::Soloud_getStreamPosition(self._inner, aVoiceHandle)
        }
    }

    pub fn pause(&mut self, aVoiceHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_getPause(self._inner, aVoiceHandle) != 0
        }
    }

    pub fn volume(&mut self, aVoiceHandle: Handle) -> f32 {
        unsafe {
            ffi::Soloud_getVolume(self._inner, aVoiceHandle)
        }
    }

    pub fn overall_volume(&mut self, aVoiceHandle: Handle) -> f32 {
        unsafe {
            ffi::Soloud_getOverallVolume(self._inner, aVoiceHandle)
        }
    }

    pub fn pan(&mut self, aVoiceHandle: Handle) -> f32 {
        unsafe {
            ffi::Soloud_getPan(self._inner, aVoiceHandle)
        }
    }

    pub fn sample_rate(&mut self, aVoiceHandle: Handle) -> f32 {
        unsafe {
            ffi::Soloud_getSamplerate(self._inner, aVoiceHandle)
        }
    }

    pub fn protect_voice(&mut self, aVoiceHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_getProtectVoice(self._inner, aVoiceHandle) != 0
        }
    }

    pub fn is_valid_voice_handle(&mut self, aVoiceHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_isValidVoiceHandle(self._inner, aVoiceHandle) != 0
        }
    }

    pub fn relative_play_speed(&mut self, aVoiceHandle: Handle) -> f32 {
        unsafe {
            ffi::Soloud_getRelativePlaySpeed(self._inner, aVoiceHandle)
        }
    }

    pub fn post_clip_scaler(&mut self) -> f32 {
        unsafe {
            ffi::Soloud_getPostClipScaler(self._inner)
        }
    }

    pub fn main_resampler(&mut self) -> u32 {
        unsafe {
            ffi::Soloud_getMainResampler(self._inner)
        }
    }

    pub fn global_volume(&mut self) -> f32 {
        unsafe {
            ffi::Soloud_getGlobalVolume(self._inner)
        }
    }

    pub fn max_active_voice_count(&mut self) -> u32 {
        unsafe {
            ffi::Soloud_getMaxActiveVoiceCount(self._inner)
        }
    }

    pub fn looping(&mut self, aVoiceHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_getLooping(self._inner, aVoiceHandle) != 0
        }
    }

    pub fn auto_stop(&mut self, aVoiceHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_getAutoStop(self._inner, aVoiceHandle) != 0
        }
    }

    pub fn loop_point(&mut self, aVoiceHandle: Handle) -> f64 {
        unsafe {
            ffi::Soloud_getLoopPoint(self._inner, aVoiceHandle)
        }
    }

    pub fn set_loop_point(&mut self, aVoiceHandle: Handle, aLoopPoint: f64) {
        unsafe {
            ffi::Soloud_setLoopPoint(self._inner, aVoiceHandle, aLoopPoint)
        }
    }

    pub fn set_looping(&mut self, aVoiceHandle: Handle, aLooping: bool) {
        unsafe {
            ffi::Soloud_setLooping(self._inner, aVoiceHandle, aLooping as i32)
        }
    }

    pub fn set_auto_stop(&mut self, aVoiceHandle: Handle, aAutoStop: bool) {
        unsafe {
            ffi::Soloud_setAutoStop(self._inner, aVoiceHandle, aAutoStop as i32)
        }
    }

    pub fn set_max_active_voice_count(&mut self, aVoiceCount: u32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_setMaxActiveVoiceCount(self._inner, aVoiceCount);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn set_inaudible_behavior(&mut self, aVoiceHandle: Handle, aMustTick: bool, aKill: bool) {
        unsafe {
            ffi::Soloud_setInaudibleBehavior(self._inner, aVoiceHandle, aMustTick as i32, aKill as i32)
        }
    }

    pub fn set_post_clip_scaler(&mut self, aScaler: f32) {
        unsafe {
            ffi::Soloud_setPostClipScaler(self._inner, aScaler)
        }
    }

    pub fn set_main_resampler(&mut self, aResampler: u32) {
        unsafe {
            ffi::Soloud_setMainResampler(self._inner, aResampler)
        }
    }

    pub fn set_pause(&mut self, aVoiceHandle: Handle, aPause: bool) {
        unsafe {
            ffi::Soloud_setPause(self._inner, aVoiceHandle, aPause as i32)
        }
    }

    pub fn set_pause_all(&mut self, aPause: bool) {
        unsafe {
            ffi::Soloud_setPauseAll(self._inner, aPause as i32)
        }
    }

    pub fn set_relative_play_speed(&mut self, aVoiceHandle: Handle, aSpeed: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_setRelativePlaySpeed(self._inner, aVoiceHandle, aSpeed);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn set_protect_voice(&mut self, aVoiceHandle: Handle, aProtect: bool) {
        unsafe {
            ffi::Soloud_setProtectVoice(self._inner, aVoiceHandle, aProtect as i32)
        }
    }

    pub fn set_samplerate(&mut self, aVoiceHandle: Handle, aSamplerate: f32) {
        unsafe {
            ffi::Soloud_setSamplerate(self._inner, aVoiceHandle, aSamplerate)
        }
    }

    pub fn set_pan(&mut self, aVoiceHandle: Handle, aPan: f32) {
        unsafe {
            ffi::Soloud_setPan(self._inner, aVoiceHandle, aPan)
        }
    }

    pub fn set_pan_absolute(&mut self, aVoiceHandle: Handle, aLVolume: f32, aRVolume: f32) {
        unsafe {
            ffi::Soloud_setPanAbsolute(self._inner, aVoiceHandle, aLVolume, aRVolume)
        }
    }

    pub fn set_channel_volume(&mut self, aVoiceHandle: Handle, aChannel: u32, aVolume: f32) {
        unsafe {
            ffi::Soloud_setChannelVolume(self._inner, aVoiceHandle, aChannel, aVolume)
        }
    }

    pub fn set_volume(&mut self, aVoiceHandle: Handle, aVolume: f32) {
        unsafe {
            ffi::Soloud_setVolume(self._inner, aVoiceHandle, aVolume)
        }
    }

    pub fn set_delay_samples(&mut self, aVoiceHandle: Handle, aSamples: u32) {
        unsafe {
            ffi::Soloud_setDelaySamples(self._inner, aVoiceHandle, aSamples)
        }
    }

    pub fn fade_volume(&mut self, aVoiceHandle: Handle, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_fadeVolume(self._inner, aVoiceHandle, aTo, aTime)
        }
    }

    pub fn fade_pan(&mut self, aVoiceHandle: Handle, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_fadePan(self._inner, aVoiceHandle, aTo, aTime)
        }
    }

    pub fn fade_relative_play_speed(&mut self, aVoiceHandle: Handle, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_fadeRelativePlaySpeed(self._inner, aVoiceHandle, aTo, aTime)
        }
    }

    pub fn fade_global_volume(&mut self, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_fadeGlobalVolume(self._inner, aTo, aTime)
        }
    }

    pub fn schedule_pause(&mut self, aVoiceHandle: Handle, aTime: f64) {
        unsafe {
            ffi::Soloud_schedulePause(self._inner, aVoiceHandle, aTime)
        }
    }

    pub fn schedule_stope(&mut self, aVoiceHandle: Handle, aTime: f64) {
        unsafe {
            ffi::Soloud_scheduleStop(self._inner, aVoiceHandle, aTime)
        }
    }

    pub fn oscillate_volume(&mut self, aVoiceHandle: Handle, aFrom: f32, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_oscillateVolume(self._inner, aVoiceHandle, aFrom, aTo, aTime)
        }
    }

    pub fn oscillate_pan(&mut self, aVoiceHandle: Handle, aFrom: f32, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_oscillatePan(self._inner, aVoiceHandle, aFrom, aTo, aTime)
        }
    }

    pub fn oscillate_relative_play_speed(&mut self, aVoiceHandle: Handle, aFrom: f32, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_oscillateRelativePlaySpeed(self._inner, aVoiceHandle, aFrom, aTo, aTime)
        }
    }

    pub fn oscillate_global_volume(&mut self, aFrom: f32, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_oscillateGlobalVolume(self._inner, aFrom, aTo, aTime)
        }
    }

    pub fn set_visualize_enable(&mut self, aEnable: bool) {
        unsafe {
            ffi::Soloud_setVisualizationEnable(self._inner, aEnable as i32)
        }
    }

    pub fn calc_FFT(&mut self) -> Vec<f32> {
        unsafe {
            let ret = ffi::Soloud_calcFFT(self._inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }

    pub fn wave(&mut self) -> Vec<f32> {
        unsafe {
            let ret = ffi::Soloud_getWave(self._inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }
    pub fn approximate_volume(&mut self, aChannel: u32) -> f32 {
        unsafe {
            ffi::Soloud_getApproximateVolume(self._inner, aChannel)
        }
    }

    pub fn loop_count(&mut self, aVoiceHandle: Handle) -> u32 {
        unsafe {
            ffi::Soloud_getLoopCount(self._inner, aVoiceHandle)
        }
    }

    pub fn info(&mut self, aVoiceHandle: Handle, aInfoKey: u32) -> f32 {
        unsafe {
            ffi::Soloud_getInfo(self._inner, aVoiceHandle, aInfoKey)
        }
    }

    pub fn create_voice_group(&mut self) -> Handle {
        unsafe {
            ffi::Soloud_createVoiceGroup(self._inner)
        }
    }

    pub fn destroy_voice_group(&mut self, aVoiceGroupHandle: Handle) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_destroyVoiceGroup(self._inner, aVoiceGroupHandle);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn add_voice_to_group(&mut self, aVoiceGroupHandle: Handle, aVoiceHandle: Handle) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_addVoiceToGroup(self._inner, aVoiceGroupHandle, aVoiceHandle);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn is_voice_group(&mut self, aVoiceGroupHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_isVoiceGroup(self._inner, aVoiceGroupHandle) != 0
        }
    }

    pub fn is_voice_group_empty(&mut self, aVoiceGroupHandle: Handle) -> bool {
        unsafe {
            ffi::Soloud_isVoiceGroupEmpty(self._inner, aVoiceGroupHandle) != 0
        }
    }

    pub fn update_3d_audio(&mut self) {
        unsafe {
            ffi::Soloud_update3dAudio(self._inner)
        }
    }

    pub fn set_3d_sound_speed(&mut self, aSpeed: f32) -> Result<(), SoloudError> {
        unsafe {
            let ret = ffi::Soloud_set3dSoundSpeed(self._inner, aSpeed);
            if ret != 0 {
                Err(SoloudError::Internal(SoloudErrorKind::from_i32(ret)))
            } else {
                Ok(())
            }
        }
    }

    pub fn get_3d_sound_speed(&mut self) -> f32 {
        unsafe {
            ffi::Soloud_get3dSoundSpeed(self._inner)
        }
    }

    pub fn set_3d_listener_params(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32, aAtX: f32, aAtY: f32, aAtZ: f32, aUpX: f32, aUpY: f32, aUpZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerParameters(self._inner, aPosX, aPosY, aPosZ, aAtX, aAtY, aAtZ, aUpX, aUpY, aUpZ)
        }
    }

    pub fn set_3d_listener_params_ex(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32, aAtX: f32, aAtY: f32, aAtZ: f32, aUpX: f32, aUpY: f32, aUpZ: f32, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerParametersEx(self._inner, aPosX, aPosY, aPosZ, aAtX, aAtY, aAtZ, aUpX, aUpY, aUpZ, aVelocityX, aVelocityY, aVelocityZ)
        }
    }

    pub fn set_3d_listener_position(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerPosition(self._inner, aPosX, aPosY, aPosZ)
        }
    }

    pub fn set_3d_listener_at(&mut self, aAtX: f32, aAtY: f32, aAtZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerAt(self._inner, aAtX, aAtY, aAtZ)
        }
    }

    pub fn set_3d_listener_up(&mut self, aUpX: f32, aUpY: f32, aUpZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerUp(self._inner, aUpX, aUpY, aUpZ)
        }
    }

    pub fn set_3d_listener_velocity(&mut self, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32) {
        unsafe {
            ffi::Soloud_set3dListenerVelocity(self._inner, aVelocityX, aVelocityY, aVelocityZ)
        }
    }

    pub fn set_3d_source_params(&mut self, aVoiceHandle: Handle, aPosX: f32, aPosY: f32, aPosZ: f32) {
        unsafe {
            ffi::Soloud_set3dSourceParameters(self._inner, aVoiceHandle, aPosX, aPosY, aPosZ)
        }
    }

    pub fn set_3d_source_params_ex(&mut self, aVoiceHandle: Handle, aPosX: f32, aPosY: f32, aPosZ: f32, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32) {
        unsafe {
            ffi::Soloud_set3dSourceParametersEx(self._inner, aVoiceHandle, aPosX, aPosY, aPosZ, aVelocityX, aVelocityY, aVelocityZ)
        }
    }

    pub fn set_3d_source_position(&mut self, aVoiceHandle: Handle, aPosX: f32, aPosY: f32, aPosZ: f32) {
        unsafe {
            ffi::Soloud_set3dSourcePosition(self._inner, aVoiceHandle, aPosX, aPosY, aPosZ)
        }
    }

    pub fn set_3d_source_velocity(&mut self, aVoiceHandle: Handle, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32) {
        unsafe {
            ffi::Soloud_set3dSourceVelocity(self._inner, aVoiceHandle, aVelocityX, aVelocityY, aVelocityZ)
        }
    }

    pub fn set_3d_source_minmax_distance(&mut self, aVoiceHandle: Handle, aMinDistance: f32, aMaxDistance: f32) {
        unsafe {
            ffi::Soloud_set3dSourceMinMaxDistance(self._inner, aVoiceHandle, aMinDistance, aMaxDistance)
        }
    }

    pub fn set_3d_source_attenuation(&mut self, aVoiceHandle: Handle, aAttenuationModel: u32, aAttenuationRolloffFactor: f32) {
        unsafe {
            ffi::Soloud_set3dSourceAttenuation(self._inner, aVoiceHandle, aAttenuationModel, aAttenuationRolloffFactor)
        }
    }

    pub fn set_3d_source_doppler_factor(&mut self, aVoiceHandle: Handle, aDopplerFactor: f32) {
        unsafe {
            ffi::Soloud_set3dSourceDopplerFactor(self._inner, aVoiceHandle, aDopplerFactor)
        }
    }

    pub fn mix(&mut self, aBuffer: &mut [f32]) {
        unsafe {
            ffi::Soloud_mix(self._inner, aBuffer.as_mut_ptr(), aBuffer.len() as u32)
        }
    }

    pub fn mix_signed_16(&mut self, aBuffer: &mut [i16]) {
        unsafe {
            ffi::Soloud_mixSigned16(self._inner, aBuffer.as_mut_ptr(), aBuffer.len() as u32)
        }
    }
    pub fn set_filter_param(&mut self, aVoiceHandle: Handle, aFilterId: u32, aAttributeId: impl FilterType, aValue: f32) {
        unsafe {
            ffi::Soloud_setFilterParameter(self._inner, aVoiceHandle, aFilterId, aAttributeId.to_u32(), aValue)
        }
    }
    
    pub fn filterParam(&mut self, aVoiceHandle: Handle, aFilterId: u32, aAttributeId: impl FilterType) -> f32 {
        unsafe {
            ffi::Soloud_getFilterParameter(self._inner, aVoiceHandle, aFilterId, aAttributeId.to_u32())
        }
    }
    
    pub fn fade_filter_param(&mut self, aVoiceHandle: Handle, aFilterId: u32, aAttributeId: impl FilterType, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_fadeFilterParameter(self._inner, aVoiceHandle, aFilterId, aAttributeId.to_u32(), aTo, aTime)
        }
    }
    
    pub fn oscillate_filter_param(&mut self, aVoiceHandle: Handle, aFilterId: u32, aAttributeId: impl FilterType, aFrom: f32, aTo: f32, aTime: f64) {
        unsafe {
            ffi::Soloud_oscillateFilterParameter(self._inner, aVoiceHandle, aFilterId, aAttributeId.to_u32(), aFrom, aTo, aTime)
        }
    }
    
    pub fn set_global_filter(&mut self, aFilterId: u32, aFilter: impl FilterExt) {
        unsafe {
            ffi::Soloud_setGlobalFilter(self._inner, aFilterId, aFilter.inner())
        }
    }
}
