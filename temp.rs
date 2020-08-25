
    pub fn initEx(&mut self, aFlags: u32, aBackend: u32, aSamplerate: u32, aBufferSize: u32, aChannels: u32) -> i32;

    pub fn getVersion(&mut self) -> u32;

    pub fn getBackendId(&mut self) -> u32;

    pub fn getBackendString(&mut self) -> String;

    pub fn getBackendChannels(&mut self) -> u32;

    pub fn getBackendSamplerate(&mut self) -> u32;

    pub fn getBackendBufferSize(&mut self) -> u32;

    pub fn setSpeakerPosition(&mut self, aChannel: u32, aX: f32, aY: f32, aZ: f32) -> Result<(), SoloudError>;

    pub fn getSpeakerPosition(&mut self, aChannel: u3) -> Result<(f32, f32, f32), SoloudError>;

    pub fn playEx<AS: AudioSource>(&mut self, aSound: &AS, aVolume: f32, aPan: f32, aPaused: i32, aBus: u32) -> u32;

    pub fn playClocked<AS: AudioSource>(&mut self, aSoundTime: f64, aSound: &AS) -> u32;

    pub fn playClockedEx<AS: AudioSource>(&mut self, aSoundTime: f64, aSound: &AS, aVolume: f32, aPan: f32, aBus: u32) -> u32;

    pub fn play3d<AS: AudioSource>(&mut self, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32) -> u32;

    pub fn play3dEx<AS: AudioSource>(&mut self, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32, aVelX: f32, aVelY: f32, aVelZ: f32, aVolume: f32, aPaused: i32, aBus: u32) -> u32;

    pub fn play3dClocked<AS: AudioSource>(&mut self, aSoundTime: f64, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32) -> u32;

    pub fn play3dClockedEx<AS: AudioSource>(&mut self, aSoundTime: f64, aSound: &AS, aPosX: f32, aPosY: f32, aPosZ: f32, aVelX: f32, aVelY: f32, aVelZ: f32, aVolume: f32, aBus: u32) -> u32;

    pub fn playBackground<AS: AudioSource>(&mut self, aSound: &AS) -> u32;

    pub fn playBackgroundEx<AS: AudioSource>(&mut self, aSound: &AS, aVolume: f32, aPaused: i32, aBus: u32) -> u32;

    pub fn seek(&mut self, aVoiceHandle: u32, aSeconds: f64) -> i32;

    pub fn stop(&mut self, aVoiceHandle: u32);

    pub fn stopAll(&mut self);

    pub fn stopAudioSource<AS: AudioSource>(&mut self, aSound: &AS);

    pub fn countAudioSource<AS: AudioSource>(&mut self, aSound: &AS) -> i32;

    pub fn setFilterParameter(&mut self, aVoiceHandle: u32, aFilterId: u32, aAttributeId: u32, aValue: f32);

    pub fn getFilterParameter(&mut self, aVoiceHandle: u32, aFilterId: u32, aAttributeId: u32) -> f32;

    pub fn fadeFilterParameter(&mut self, aVoiceHandle: u32, aFilterId: u32, aAttributeId: u32, aTo: f32, aTime: f64);

    pub fn oscillateFilterParameter(&mut self, aVoiceHandle: u32, aFilterId: u32, aAttributeId: u32, aFrom: f32, aTo: f32, aTime: f64);

    pub fn getStreamTime(&mut self, aVoiceHandle: u32) -> f64;

    pub fn getStreamPosition(&mut self, aVoiceHandle: u32) -> f64;

    pub fn getPause(&mut self, aVoiceHandle: u32) -> i32;

    pub fn getVolume(&mut self, aVoiceHandle: u32) -> f32;

    pub fn getOverallVolume(&mut self, aVoiceHandle: u32) -> f32;

    pub fn getPan(&mut self, aVoiceHandle: u32) -> f32;

    pub fn getSamplerate(&mut self, aVoiceHandle: u32) -> f32;

    pub fn getProtectVoice(&mut self, aVoiceHandle: u32) -> i32;

    pub fn isValidVoiceHandle(&mut self, aVoiceHandle: u32) -> i32;

    pub fn getRelativePlaySpeed(&mut self, aVoiceHandle: u32) -> f32;

    pub fn getPostClipScaler(&mut self) -> f32;

    pub fn getMainResampler(&mut self) -> u32;

    pub fn getGlobalVolume(&mut self) -> f32;

    pub fn getMaxActiveVoiceCount(&mut self) -> u32;

    pub fn getLooping(&mut self, aVoiceHandle: u32) -> i32;

    pub fn getAutoStop(&mut self, aVoiceHandle: u32) -> i32;

    pub fn getLoopPoint(&mut self, aVoiceHandle: u32) -> f64;

    pub fn setLoopPoint(&mut self, aVoiceHandle: u32, aLoopPoint: f64);

    pub fn setLooping(&mut self, aVoiceHandle: u32, aLooping: i32);

    pub fn setAutoStop(&mut self, aVoiceHandle: u32, aAutoStop: i32);

    pub fn setMaxActiveVoiceCount(&mut self, aVoiceCount: u32) -> i32;

    pub fn setInaudibleBehavior(&mut self, aVoiceHandle: u32, aMustTick: i32, aKill: i32);

    pub fn setPostClipScaler(&mut self, aScaler: f32);

    pub fn setMainResampler(&mut self, aResampler: u32);

    pub fn setPause(&mut self, aVoiceHandle: u32, aPause: i32);

    pub fn setPauseAll(&mut self, aPause: i32);

    pub fn setRelativePlaySpeed(&mut self, aVoiceHandle: u32, aSpeed: f32) -> i32;

    pub fn setProtectVoice(&mut self, aVoiceHandle: u32, aProtect: i32);

    pub fn setSamplerate(&mut self, aVoiceHandle: u32, aSamplerate: f32);

    pub fn setPan(&mut self, aVoiceHandle: u32, aPan: f32);

    pub fn setPanAbsolute(&mut self, aVoiceHandle: u32, aLVolume: f32, aRVolume: f32);

    pub fn setChannelVolume(&mut self, aVoiceHandle: u32, aChannel: u32, aVolume: f32);

    pub fn setVolume(&mut self, aVoiceHandle: u32, aVolume: f32);

    pub fn setDelaySamples(&mut self, aVoiceHandle: u32, aSamples: u32);

    pub fn fadeVolume(&mut self, aVoiceHandle: u32, aTo: f32, aTime: f64);

    pub fn fadePan(&mut self, aVoiceHandle: u32, aTo: f32, aTime: f64);

    pub fn fadeRelativePlaySpeed(&mut self, aVoiceHandle: u32, aTo: f32, aTime: f64);

    pub fn fadeGlobalVolume(&mut self, aTo: f32, aTime: f64);

    pub fn schedulePause(&mut self, aVoiceHandle: u32, aTime: f64);

    pub fn scheduleStop(&mut self, aVoiceHandle: u32, aTime: f64);

    pub fn oscillateVolume(&mut self, aVoiceHandle: u32, aFrom: f32, aTo: f32, aTime: f64);

    pub fn oscillatePan(&mut self, aVoiceHandle: u32, aFrom: f32, aTo: f32, aTime: f64);

    pub fn oscillateRelativePlaySpeed(&mut self, aVoiceHandle: u32, aFrom: f32, aTo: f32, aTime: f64);

    pub fn oscillateGlobalVolume(&mut self, aFrom: f32, aTo: f32, aTime: f64);

    pub fn setGlobalFilter(&mut self, aFilterId: u32, aFilter: &Filter);

    pub fn setVisualizationEnable(&mut self, aEnable: i32);

    pub fn calcFFT(&mut self) -> Vec<f32>;

    pub fn getWave(&mut self) -> Vec<f32>;

    pub fn getApproximateVolume(&mut self, aChannel: u32) -> f32;

    pub fn getLoopCount(&mut self, aVoiceHandle: u32) -> u32;

    pub fn getInfo(&mut self, aVoiceHandle: u32, aInfoKey: u32) -> f32;

    pub fn createVoiceGroup(&mut self) -> u32;

    pub fn destroyVoiceGroup(&mut self, aVoiceGroupHandle: u32) -> i32;

    pub fn addVoiceToGroup(&mut self, aVoiceGroupHandle: u32, aVoiceHandle: u32) -> i32;

    pub fn isVoiceGroup(&mut self, aVoiceGroupHandle: u32) -> i32;

    pub fn isVoiceGroupEmpty(&mut self, aVoiceGroupHandle: u32) -> i32;

    pub fn update3dAudio(&mut self);

    pub fn set3dSoundSpeed(&mut self, aSpeed: f32) -> i32;

    pub fn get3dSoundSpeed(&mut self) -> f32;

    pub fn set3dListenerParameters(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32, aAtX: f32, aAtY: f32, aAtZ: f32, aUpX: f32, aUpY: f32, aUpZ: f32);

    pub fn set3dListenerParametersEx(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32, aAtX: f32, aAtY: f32, aAtZ: f32, aUpX: f32, aUpY: f32, aUpZ: f32, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32);

    pub fn set3dListenerPosition(&mut self, aPosX: f32, aPosY: f32, aPosZ: f32);

    pub fn set3dListenerAt(&mut self, aAtX: f32, aAtY: f32, aAtZ: f32);

    pub fn set3dListenerUp(&mut self, aUpX: f32, aUpY: f32, aUpZ: f32);

    pub fn set3dListenerVelocity(&mut self, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32);

    pub fn set3dSourceParameters(&mut self, aVoiceHandle: u32, aPosX: f32, aPosY: f32, aPosZ: f32);

    pub fn set3dSourceParametersEx(&mut self, aVoiceHandle: u32, aPosX: f32, aPosY: f32, aPosZ: f32, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32);

    pub fn set3dSourcePosition(&mut self, aVoiceHandle: u32, aPosX: f32, aPosY: f32, aPosZ: f32);

    pub fn set3dSourceVelocity(&mut self, aVoiceHandle: u32, aVelocityX: f32, aVelocityY: f32, aVelocityZ: f32);

    pub fn set3dSourceMinMaxDistance(&mut self, aVoiceHandle: u32, aMinDistance: f32, aMaxDistance: f32);

    pub fn set3dSourceAttenuation(&mut self, aVoiceHandle: u32, aAttenuationModel: u32, aAttenuationRolloffFactor: f32);

    pub fn set3dSourceDopplerFactor(&mut self, aVoiceHandle: u32, aDopplerFactor: f32);

    pub fn mix(&mut self, aBuffer: &mut [f32]);

    pub fn mixSigned16(&mut self, aBuffer: &mut [i16]);