//! # soloud
//!
//! Rust bindings for the soloud audio engine library.
//!
//! Supported formats: wav, mp3, ogg. The library also comes with a speech synthesizer.
//!
//!
//! - The official soloud [website](https://sol.gfxile.net/soloud/index.html)
//! - The official soloud [repo](https://github.com/jarikomppa/soloud)
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! soloud = "0.3"
//! ```
//!
//! Or to use the latest developments:
//! ```toml
//! [dependencies]
//! soloud = { git = "https://!github.com/moalyousef/soloud-rs" }
//! ```
//!
//! To play audio:
//! ```no_run
//! use soloud::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut sl = Soloud::default()?;
//!
//!     let mut wav = audio::Wav::default();
//!
//!     wav.load(&std::path::Path::new("sample.wav"))?;
//!
//!     sl.play(&wav);
//!     while sl.voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     wav.load(&std::path::Path::new("Recording.mp3"))?;
//!
//!     sl.play(&wav);
//!     while sl.voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! To play speech:
//! ```no_run
//! use soloud::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut sl = Soloud::default()?;
//!
//!     let mut speech = audio::Speech::default();
//!
//!     speech.set_text("Hello World")?;
//!
//!     sl.play(&speech);
//!     while sl.active_voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     speech.set_text("1 2 3")?;
//!
//!     sl.play(&speech);
//!     while sl.active_voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     speech.set_text("Can you hear me?")?;
//!
//!     sl.play(&speech);
//!     while sl.active_voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! To add a filter:
//! ```no_run
//! use soloud::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut sl = Soloud::default()?;
//!
//!     let mut wav = audio::Wav::default();
//!     let mut filt = filter::EchoFilter::default();
//!     filt.set_params(0.2)?;
//!
//!     wav.load(&std::path::Path::new("sample.wav"))?;
//!     wav.set_filter(0, Some(&filt));
//!
//!     sl.play(&wav);
//!     while sl.voice_count() > 0 {
//!         std::thread::sleep(std::time::Duration::from_millis(100));
//!     }
//!
//!     Ok(())
//! }
//! ```
//! ## Backends
//! The default backend is miniaudio, however Soloud supports several backends to varying degrees. To enable support of a certain backend, alsa for example:
//! ```toml
//! [dependencies]
//! soloud = { version = "0.3", default-features = false, features = ["alsa"] }
//! ```
//! This also assumes that those libraries headers are in your include path where CMake can find them, otherwise you can set it via the command line (posix):
//! ```sh
//! $ export CXXFLAGS="-I /path/to/include"
//! ```
//! or for Windows:
//! ```sh
//! $ set CXXFLAGS="-I C:\\path\\to\\include"
//! ```
//! The same can be done for CFLAGS if needed.
//!
//! ### Supported backends:
//! - miniaudio
//! - oss
//! - alsa
//! - sdl2
//! - sdl2-static
//! - portaudio
//! - openal
//! - xaudio2
//! - winmm
//! - wasapi
//! - opensles
//! - coreaudio
//! - jack

#![allow(unused_unsafe)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]

/// FFI function call with error handling
///
/// NOTE: It has to be defined _before_ declaring sub modules.
macro_rules! ffi_call {
    ($s:expr) => {
        match unsafe { $s } {
            0 => Ok(()),
            code => Err(SoloudError::Internal(SoloudErrorKind::from_i32(code))),
        }
    };
}

/// Audio module containing all audio types
pub mod audio;
/// Filter module containing all filter types and attributes
pub mod filter;
/// Prelude module containing all traits and error codes
pub mod prelude;
// pub mod effects;

#[macro_use]
extern crate soloud_derive;

#[macro_use]
extern crate bitflags;

pub use audio::*;
pub use filter::*;
pub use prelude::*;

use soloud_sys::soloud as ffi;

/// A wrapper around the raw handle of the audio object which can be used to adjust the parameters
/// of the sound while it's playing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle(u32);

impl Handle {
    /// Primary bus
    pub const PRIMARY: Self = Self(0);

    /// Create a handle from a raw value
    /// # Safety
    /// The value must be a valid handle
    pub unsafe fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Retrieve the raw value of the handle
    pub fn raw(&self) -> u32 {
        self.0
    }
}

/// Backends supported by the Soloud crate
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Backend {
    /// Autoselection by Soloud
    Auto = 0,
    /// Sdl2
    Sdl2 = 2,
    /// Portaudio
    Portaudio = 3,
    /// Winmm
    Winmm = 4,
    /// Xaudio2
    Xaudio2 = 5,
    /// Wasapi
    Wasapi = 6,
    /// Alsa
    Alsa = 7,
    /// Jack
    Jack = 8,
    /// Oss
    Oss = 9,
    /// OpenAL
    OpenAL = 10,
    /// Coreaudio
    CoreAudio = 11,
    /// OpenSLES
    OpenSLES = 12,
    /// Miniaudio
    Miniaudio = 14,
    /// Nosound
    Nosound = 15,
    /// Null driver
    Null = 16,
}

/// Wrapper around the Soloud native object
#[derive(Debug)]
pub struct Soloud {
    _inner: *mut ffi::Soloud,
}

unsafe impl Send for Soloud {}

unsafe impl Sync for Soloud {}

impl Soloud {
    /// Creates an uninitialized instance of a Soloud engine
    pub fn default_uninit() -> std::mem::MaybeUninit<Self> {
        unsafe {
            let ptr = ffi::Soloud_create();
            assert!(!ptr.is_null());
            std::mem::MaybeUninit::new(Soloud { _inner: ptr })
        }
    }

    /// initialize an uninitialized instance of Soloud
    pub fn init(&mut self) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_init(self._inner))
    }

    /// Creates a default initialized instance of soloud
    pub fn default() -> Result<Self, SoloudError> {
        let mut temp = unsafe { Soloud::default_uninit().assume_init() };
        temp.init()?;
        Ok(temp)
    }

    /// initialize an uninitialized instance of Soloud with extra args
    pub fn init_ex(
        &mut self,
        flags: SoloudFlag,
        backend: Backend,
        samplerate: u32,
        buf_size: u32,
        channels: u32,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_initEx(
            self._inner,
            flags.bits() as u32,
            backend as u32,
            samplerate,
            buf_size,
            channels,
        ))
    }

    /// Creates a default initialized instance of soloud
    pub fn new(
        flags: SoloudFlag,
        backend: Backend,
        samplerate: u32,
        buf_size: u32,
        channels: u32,
    ) -> Result<Self, SoloudError> {
        let mut temp = unsafe { Soloud::default_uninit().assume_init() };
        temp.init_ex(flags, backend, samplerate, buf_size, channels)?;
        Ok(temp)
    }

    /// Gets the current version of the Soloud library
    pub fn version(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getVersion(self._inner) }
    }

    /// Gets the backend id
    pub fn backend_id(&self) -> u32 {
        unsafe { ffi::Soloud_getBackendId(self._inner) }
    }

    /// Gets the backend name
    pub fn backend_string(&self) -> String {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = ffi::Soloud_getBackendString(self._inner);
            assert!(!ptr.is_null());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().to_string()
        }
    }

    /// Get the backend channels
    pub fn backend_channels(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getBackendChannels(self._inner) }
    }

    /// Get the backend samplerate
    pub fn backend_samplerate(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getBackendSamplerate(self._inner) }
    }

    /// Get the backend buffer size
    pub fn backend_buffer_size(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getBackendBufferSize(self._inner) }
    }

    /// Set speaker position
    pub fn set_speaker_position(
        &mut self,
        channel: u32,
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_setSpeakerPosition(
            self._inner,
            channel,
            x,
            y,
            z
        ))
    }

    /// Get the speaker position
    pub fn speaker_position(&self, channel: u32) -> Result<(f32, f32, f32), SoloudError> {
        assert!(!self._inner.is_null());
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        ffi_call!(ffi::Soloud_getSpeakerPosition(
            self._inner,
            channel,
            &mut x,
            &mut y,
            &mut z
        ))?;
        Ok((x, y, z))
    }

    /// Play audio with extra args
    pub fn play_ex<AS: AudioExt>(
        &self,
        sound: &AS,
        volume: f32,
        pan: f32,
        paused: bool,
        bus: Handle,
    ) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_playEx(
                self._inner,
                sound.inner(),
                volume,
                pan,
                paused as i32,
                bus.0,
            )
        })
    }

    /// Play clocked
    pub fn play_clocked<AS: AudioExt>(&self, sound_time: f64, sound: &AS) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe { ffi::Soloud_playClocked(self._inner, sound_time, sound.inner()) })
    }

    /// Play clocked with extra args
    pub fn play_clocked_ex<AS: AudioExt>(
        &self,
        sound_time: f64,
        sound: &AS,
        volume: f32,
        pan: f32,
        bus: Handle,
    ) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_playClockedEx(self._inner, sound_time, sound.inner(), volume, pan, bus.0)
        })
    }

    /// Play 3D
    pub fn play_3d<AS: AudioExt>(&self, sound: &AS, pos_x: f32, pos_y: f32, pos_z: f32) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe { ffi::Soloud_play3d(self._inner, sound.inner(), pos_x, pos_y, pos_z) })
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
        bus: Handle,
    ) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_play3dEx(
                self._inner,
                sound.inner(),
                pos_x,
                pos_y,
                pos_z,
                vel_x,
                vel_y,
                vel_z,
                volume,
                paused as i32,
                bus.0,
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
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_play3dClocked(self._inner, sound_time, sound.inner(), pos_x, pos_y, pos_z)
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
        bus: Handle,
    ) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_play3dClockedEx(
                self._inner,
                sound_time,
                sound.inner(),
                pos_x,
                pos_y,
                pos_z,
                vel_x,
                vel_y,
                vel_z,
                volume,
                bus.0,
            )
        })
    }

    /// Play in the background
    pub fn play_background<AS: AudioExt>(&self, sound: &AS) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe { ffi::Soloud_playBackground(self._inner, sound.inner()) })
    }

    /// Play in the background with extra args
    pub fn play_background_ex<AS: AudioExt>(
        &self,
        sound: &AS,
        volume: f32,
        paused: bool,
        bus: Handle,
    ) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe {
            ffi::Soloud_playBackgroundEx(self._inner, sound.inner(), volume, paused as i32, bus.0)
        })
    }

    /// Seek in seconds
    pub fn seek(&self, voice_handle: Handle, seconds: f64) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_seek(self._inner, voice_handle.0, seconds))
    }

    /// Stop audio by handle
    pub fn stop(&self, voice_handle: Handle) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_stop(self._inner, voice_handle.0) }
    }

    /// Stop all audio
    pub fn stop_all(&self) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_stopAll(self._inner) }
    }

    /// Deinitialize the soloud engine
    pub(crate) fn deinit(&mut self) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_deinit(self._inner);
            self._inner = std::ptr::null_mut();
        }
    }

    /// Play audio, returns a handle identifying the played audio
    pub fn play<T: AudioExt>(&self, sound: &T) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe { ffi::Soloud_play(self._inner, sound.inner()) })
    }

    /// Get active voice count
    pub fn active_voice_count(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getActiveVoiceCount(self._inner) }
    }

    /// Get voice count
    pub fn voice_count(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getVoiceCount(self._inner) }
    }

    /// Set global volume
    pub fn set_global_volume(&mut self, val: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setGlobalVolume(self._inner, val) }
    }

    /// Stop audio source
    pub fn stop_audio_source<AS: AudioExt>(&self, sound: &AS) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_stopAudioSource(self._inner, sound.inner()) }
    }

    /// Count audio source
    pub fn count_audio_source<AS: AudioExt>(&self, sound: &AS) -> i32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_countAudioSource(self._inner, sound.inner()) }
    }

    /// Get stream time
    pub fn stream_time(&self, voice_handle: Handle) -> f64 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getStreamTime(self._inner, voice_handle.0) }
    }

    /// Get stream position
    pub fn stream_position(&self, voice_handle: Handle) -> f64 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getStreamPosition(self._inner, voice_handle.0) }
    }

    /// Pause audio
    pub fn pause(&self, voice_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getPause(self._inner, voice_handle.0) != 0 }
    }

    /// Get audio volume
    pub fn volume(&self, voice_handle: Handle) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getVolume(self._inner, voice_handle.0) }
    }

    /// Get overall volume
    pub fn overall_volume(&self, voice_handle: Handle) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getOverallVolume(self._inner, voice_handle.0) }
    }

    /// Get pan value
    pub fn pan(&self, voice_handle: Handle) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getPan(self._inner, voice_handle.0) }
    }

    /// Get samplerate of audio
    pub fn samplerate(&self, voice_handle: Handle) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getSamplerate(self._inner, voice_handle.0) }
    }

    /// Return whether protect voice is set
    pub fn protect_voice(&self, voice_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getProtectVoice(self._inner, voice_handle.0) != 0 }
    }

    /// Check whether a handle is a valid voice handle
    pub fn is_valid_voice_handle(&self, voice_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_isValidVoiceHandle(self._inner, voice_handle.0) != 0 }
    }

    /// Get relative play speed
    pub fn relative_play_speed(&self, voice_handle: Handle) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getRelativePlaySpeed(self._inner, voice_handle.0) }
    }

    /// Get post clip scaler
    pub fn post_clip_scaler(&self) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getPostClipScaler(self._inner) }
    }

    /// Get main resampler
    pub fn main_resampler(&self) -> Resampler {
        assert!(!self._inner.is_null());
        unsafe { std::mem::transmute(ffi::Soloud_getMainResampler(self._inner)) }
    }

    /// Get global volume
    pub fn global_volume(&self) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getGlobalVolume(self._inner) }
    }

    /// Get max active voice count
    pub fn max_active_voice_count(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getMaxActiveVoiceCount(self._inner) }
    }

    /// Return whether an audio is looping
    pub fn looping(&self, voice_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getLooping(self._inner, voice_handle.0) != 0 }
    }

    /// Check whether an audio auto stops
    pub fn auto_stop(&self, voice_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getAutoStop(self._inner, voice_handle.0) != 0 }
    }

    /// Get loop point
    pub fn loop_point(&self, voice_handle: Handle) -> f64 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getLoopPoint(self._inner, voice_handle.0) }
    }

    /// Set loop point
    pub fn set_loop_point(&mut self, voice_handle: Handle, loop_point: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setLoopPoint(self._inner, voice_handle.0, loop_point) }
    }

    /// Set whether audio is looping
    pub fn set_looping(&mut self, voice_handle: Handle, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setLooping(self._inner, voice_handle.0, flag as i32) }
    }

    /// Set auto stop
    pub fn set_auto_stop(&mut self, voice_handle: Handle, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setAutoStop(self._inner, voice_handle.0, flag as i32) }
    }

    /// Set max active voice count
    pub fn set_max_active_voice_count(&mut self, count: u32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_setMaxActiveVoiceCount(self._inner, count))
    }

    /// Set inaudible behaviour
    pub fn set_inaudible_behavior(&mut self, voice_handle: Handle, must_tick: bool, kill: bool) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_setInaudibleBehavior(
                self._inner,
                voice_handle.0,
                must_tick as i32,
                kill as i32,
            )
        }
    }

    /// Set post clip scaler
    pub fn set_post_clip_scaler(&mut self, scaler: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setPostClipScaler(self._inner, scaler) }
    }

    /// Set main resampler
    pub fn set_main_resampler(&mut self, resampler: Resampler) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setMainResampler(self._inner, resampler as u32) }
    }

    /// Set whether a handle pauses
    pub fn set_pause(&mut self, voice_handle: Handle, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setPause(self._inner, voice_handle.0, flag as i32) }
    }

    /// Set pause for all handles
    pub fn set_pause_all(&mut self, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setPauseAll(self._inner, flag as i32) }
    }

    /// Set relative play speed
    pub fn set_relative_play_speed(
        &mut self,
        voice_handle: Handle,
        speed: f32,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_setRelativePlaySpeed(
            self._inner,
            voice_handle.0,
            speed
        ))
    }

    /// Set whether an audio source has protect voice
    pub fn set_protect_voice(&mut self, voice_handle: Handle, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setProtectVoice(self._inner, voice_handle.0, flag as i32) }
    }

    /// Set samplerate
    pub fn set_samplerate(&mut self, voice_handle: Handle, samplerate: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setSamplerate(self._inner, voice_handle.0, samplerate) }
    }

    /// Set pan
    pub fn set_pan(&mut self, voice_handle: Handle, pan: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setPan(self._inner, voice_handle.0, pan) }
    }

    /// Set pan absolute
    pub fn set_pan_absolute(&mut self, voice_handle: Handle, lvolume: f32, rvolume: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setPanAbsolute(self._inner, voice_handle.0, lvolume, rvolume) }
    }

    /// Set channel volume
    pub fn set_channel_volume(&mut self, voice_handle: Handle, channel: u32, volume: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setChannelVolume(self._inner, voice_handle.0, channel, volume) }
    }

    /// Set volume by handle
    pub fn set_volume(&mut self, voice_handle: Handle, volume: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setVolume(self._inner, voice_handle.0, volume) }
    }

    /// Set delay samples
    pub fn set_delay_samples(&mut self, voice_handle: Handle, samples: u32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setDelaySamples(self._inner, voice_handle.0, samples) }
    }

    /// Set up volume fader
    pub fn fade_volume(&self, voice_handle: Handle, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_fadeVolume(self._inner, voice_handle.0, to, time) }
    }

    /// Set up panning fader
    pub fn fade_pan(&self, voice_handle: Handle, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_fadePan(self._inner, voice_handle.0, to, time) }
    }

    /// Set fader relative play speed
    pub fn fade_relative_play_speed(&self, voice_handle: Handle, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_fadeRelativePlaySpeed(self._inner, voice_handle.0, to, time) }
    }

    /// Set fader global volume
    pub fn fade_global_volume(&self, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_fadeGlobalVolume(self._inner, to, time) }
    }

    /// Schedule a pause
    pub fn schedule_pause(&self, voice_handle: Handle, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_schedulePause(self._inner, voice_handle.0, time) }
    }

    /// Schedule a stop
    pub fn schedule_stop(&self, voice_handle: Handle, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_scheduleStop(self._inner, voice_handle.0, time) }
    }

    /// Set up volume oscillator
    pub fn oscillate_volume(&self, voice_handle: Handle, from: f32, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_oscillateVolume(self._inner, voice_handle.0, from, to, time) }
    }

    /// Set up panning oscillator
    pub fn oscillate_pan(&self, voice_handle: Handle, from: f32, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_oscillatePan(self._inner, voice_handle.0, from, to, time) }
    }

    /// Oscillator relative play speed
    pub fn oscillate_relative_play_speed(
        &self,
        voice_handle: Handle,
        from: f32,
        to: f32,
        time: f64,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_oscillateRelativePlaySpeed(self._inner, voice_handle.0, from, to, time)
        }
    }

    /// Get oscillator global volume
    pub fn oscillate_global_volume(&self, from: f32, to: f32, time: f64) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_oscillateGlobalVolume(self._inner, from, to, time) }
    }

    /// Enable visualizations
    pub fn set_visualize_enable(&self, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setVisualizationEnable(self._inner, flag as i32) }
    }

    /// Calculate and get 256 floats of FFT data for visualization. Visualization has to be enabled before use
    pub fn calc_fft(&self) -> Vec<f32> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = ffi::Soloud_calcFFT(self._inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }

    /// Get 256 floats of wave data for visualization. Visualization has to be enabled before use
    pub fn wave(&self) -> Vec<f32> {
        assert!(!self._inner.is_null());
        unsafe {
            let ret = ffi::Soloud_getWave(self._inner);
            let ret = std::slice::from_raw_parts(ret, 256);
            ret.to_vec()
        }
    }

    /// Get approximate volume
    pub fn approximate_volume(&self, channel: u32) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getApproximateVolume(self._inner, channel) }
    }

    /// Get loop count
    pub fn loop_count(&self, voice_handle: Handle) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getLoopCount(self._inner, voice_handle.0) }
    }

    /// get info by key
    pub fn info(&self, voice_handle: Handle, key: u32) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_getInfo(self._inner, voice_handle.0, key) }
    }

    /// Create a voice group
    pub fn create_voice_group(&self) -> Handle {
        assert!(!self._inner.is_null());
        Handle(unsafe { ffi::Soloud_createVoiceGroup(self._inner) })
    }

    /// Destroy a voice group
    pub fn destroy_voice_group(&self, voice_group_handle: Handle) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_destroyVoiceGroup(
            self._inner,
            voice_group_handle.0
        ))
    }

    /// Add a voice handle to a voice group
    pub fn add_voice_to_group(
        &self,
        voice_group_handle: Handle,
        voice_handle: Handle,
    ) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_addVoiceToGroup(
            self._inner,
            voice_group_handle.0,
            voice_handle.0
        ))
    }

    /// Check whether a handle is of a voice group
    pub fn is_voice_group(&self, voice_group_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_isVoiceGroup(self._inner, voice_group_handle.0) != 0 }
    }

    /// Check whether a voice group is empty
    pub fn is_voice_group_empty(&self, voice_group_handle: Handle) -> bool {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_isVoiceGroupEmpty(self._inner, voice_group_handle.0) != 0 }
    }

    /// Update 3D audio
    pub fn update_3d_audio(&self) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_update3dAudio(self._inner) }
    }

    /// Set 3D sound speed
    pub fn set_3d_sound_speed(&self, speed: f32) -> Result<(), SoloudError> {
        assert!(!self._inner.is_null());
        ffi_call!(ffi::Soloud_set3dSoundSpeed(self._inner, speed))
    }

    /// Get 3d sound speed
    pub fn get_3d_sound_speed(&self) -> f32 {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_get3dSoundSpeed(self._inner) }
    }

    /// Set 3D listener parameters
    pub fn set_3d_listener_params(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        at_x: f32,
        at_y: f32,
        at_z: f32,
        up_x: f32,
        up_y: f32,
        up_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dListenerParameters(
                self._inner,
                pos_x,
                pos_y,
                pos_z,
                at_x,
                at_y,
                at_z,
                up_x,
                up_y,
                up_z,
            )
        }
    }

    /// Set 3D listerner parameters with extra args
    pub fn set_3d_listener_params_ex(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        at_x: f32,
        at_y: f32,
        at_z: f32,
        up_x: f32,
        up_y: f32,
        up_z: f32,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dListenerParametersEx(
                self._inner,
                pos_x,
                pos_y,
                pos_z,
                at_x,
                at_y,
                at_z,
                up_x,
                up_y,
                up_z,
                velocity_x,
                velocity_y,
                velocity_z,
            )
        }
    }

    /// Set 3D listener position
    pub fn set_3d_listener_position(&mut self, pos_x: f32, pos_y: f32, pos_z: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_set3dListenerPosition(self._inner, pos_x, pos_y, pos_z) }
    }

    /// Set 3D listener position with extra params
    pub fn set_3d_listener_at(&mut self, at_x: f32, at_y: f32, at_z: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_set3dListenerAt(self._inner, at_x, at_y, at_z) }
    }

    /// Set up 3D listener
    pub fn set_3d_listener_up(&mut self, up_x: f32, up_y: f32, up_z: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_set3dListenerUp(self._inner, up_x, up_y, up_z) }
    }

    /// Set 3D listener velocity
    pub fn set_3d_listener_velocity(&mut self, velocity_x: f32, velocity_y: f32, velocity_z: f32) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dListenerVelocity(self._inner, velocity_x, velocity_y, velocity_z)
        }
    }

    /// Set 3D source parameters
    pub fn set_3d_source_params(
        &mut self,
        voice_handle: Handle,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dSourceParameters(self._inner, voice_handle.0, pos_x, pos_y, pos_z)
        }
    }

    /// Set 3D source parameters
    pub fn set_3d_source_params_ex(
        &mut self,
        voice_handle: Handle,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dSourceParametersEx(
                self._inner,
                voice_handle.0,
                pos_x,
                pos_y,
                pos_z,
                velocity_x,
                velocity_y,
                velocity_z,
            )
        }
    }

    /// Set 3D source position
    pub fn set_3d_source_position(
        &mut self,
        voice_handle: Handle,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_set3dSourcePosition(self._inner, voice_handle.0, pos_x, pos_y, pos_z) }
    }

    /// Set 3D source velocity
    pub fn set_3d_source_velocity(
        &mut self,
        voice_handle: Handle,
        velocity_x: f32,
        velocity_y: f32,
        velocity_z: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dSourceVelocity(
                self._inner,
                voice_handle.0,
                velocity_x,
                velocity_y,
                velocity_z,
            )
        }
    }

    /// Set 3D source min and max distances
    pub fn set_3d_source_minmax_distance(
        &mut self,
        voice_handle: Handle,
        min_distance: f32,
        max_distance: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dSourceMinMaxDistance(
                self._inner,
                voice_handle.0,
                min_distance,
                max_distance,
            )
        }
    }

    /// Set 3D source attenuation
    pub fn set_3d_source_attenuation(
        &mut self,
        voice_handle: Handle,
        model: AttenuationModel,
        rolloff_factor: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_set3dSourceAttenuation(
                self._inner,
                voice_handle.0,
                model as u32,
                rolloff_factor,
            )
        }
    }

    /// Set 3D source doppler factor
    pub fn set_3d_source_doppler_factor(&mut self, voice_handle: Handle, doppler_factor: f32) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_set3dSourceDopplerFactor(self._inner, voice_handle.0, doppler_factor) }
    }

    #[allow(dead_code)]
    fn mix(&mut self, buffer: &mut [f32]) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_mix(self._inner, buffer.as_mut_ptr(), buffer.len() as u32) }
    }

    #[allow(dead_code)]
    fn mix_signed_16(&mut self, buffer: &mut [i16]) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_mixSigned16(self._inner, buffer.as_mut_ptr(), buffer.len() as u32) }
    }

    /// Set filter parameters
    pub fn set_filter_param(
        &mut self,
        voice_handle: Handle,
        filter_id: u32,
        attr: impl FilterAttr,
        val: f32,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_setFilterParameter(
                self._inner,
                voice_handle.0,
                filter_id,
                attr.to_u32(),
                val,
            )
        }
    }

    /// Get filter parameter by filter id
    pub fn filter_param(
        &mut self,
        voice_handle: Handle,
        filter_id: u32,
        attr: impl FilterAttr,
    ) -> f32 {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_getFilterParameter(self._inner, voice_handle.0, filter_id, attr.to_u32())
        }
    }

    /// Get fader filter params
    pub fn fade_filter_param(
        &mut self,
        voice_handle: Handle,
        filter_id: u32,
        attr: impl FilterAttr,
        to: f32,
        time: f64,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_fadeFilterParameter(
                self._inner,
                voice_handle.0,
                filter_id,
                attr.to_u32(),
                to,
                time,
            )
        }
    }

    /// Get oscillator filter params
    pub fn oscillate_filter_param(
        &mut self,
        voice_handle: Handle,
        filter_id: u32,
        attr: impl FilterAttr,
        from: f32,
        to: f32,
        time: f64,
    ) {
        assert!(!self._inner.is_null());
        unsafe {
            ffi::Soloud_oscillateFilterParameter(
                self._inner,
                voice_handle.0,
                filter_id,
                attr.to_u32(),
                from,
                to,
                time,
            )
        }
    }

    /// Set a global filter
    pub fn set_global_filter(&mut self, filter_id: u32, filter: impl FilterExt) {
        assert!(!self._inner.is_null());
        unsafe { ffi::Soloud_setGlobalFilter(self._inner, filter_id, filter.inner()) }
    }

    /// Get the inner pointer of the Soloud engine
    /// # Safety
    /// The pointer must remain valid
    pub unsafe fn inner(&self) -> *mut ffi::Soloud {
        self._inner
    }

    // unsafe fn from_ptr(ptr: *mut std::os::raw::c_void) -> Soloud {
    //     assert!(!ptr.is_null());
    //     Soloud {
    //         _inner: ptr as *mut ffi::Soloud,
    //     }
    // }
}

impl Drop for Soloud {
    fn drop(&mut self) {
        if !self._inner.is_null() {
            self.deinit()
        }
    }
}
