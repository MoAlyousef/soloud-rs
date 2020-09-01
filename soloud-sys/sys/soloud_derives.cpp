#include "soloud.h"
#include "soloud_audiosource.h"

#include "soloud_derives.h"

struct AudioCollider_Derived : public SoLoud::AudioCollider {
    void *ev_data_ = NULL;
    typedef float (*handler)(SoLoud::Soloud *, SoLoud::AudioSourceInstance3dData *, int,
                             void *data);
    handler inner_handler = NULL;
    void set_handler(handler h) {
        inner_handler = h;
    }
    void set_handler_data(void *data) {
        ev_data_ = data;
    }
    virtual float collide(SoLoud::Soloud *aSoloud,
                          SoLoud::AudioSourceInstance3dData *aAudioInstance3dData,
                          int aUserData) override {
        if (ev_data_ && inner_handler)
            return 0;
        return inner_handler(aSoloud, aAudioInstance3dData, aUserData, ev_data_);
    }
};

struct AudioAttenuator_Derived : public SoLoud::AudioAttenuator {
    void *ev_data_ = NULL;
    typedef float (*handler)(float, float, float, float, void *data);
    handler inner_handler = NULL;
    void set_handler(handler h) {
        inner_handler = h;
    }
    void set_handler_data(void *data) {
        ev_data_ = data;
    }
    virtual float attenuate(float aDistance, float aMinDistance, float aMaxDistance,
                            float aRolloffFactor) override {
        if (ev_data_ && inner_handler)
            return 0;
        return inner_handler(aDistance, aMinDistance, aMaxDistance, aRolloffFactor, ev_data_);
    }
};

AudioSourceInstance3dData *AudioSourceInstance3dData_new(AudioSource *as) {
    auto temp = new SoLoud::AudioSourceInstance3dData;
    temp->init(*(SoLoud::AudioSource *)as);
    return (AudioSourceInstance3dData *)temp;
}

void AudioSourceInstance3dData_delete(AudioSourceInstance3dData *inst) {
    delete (SoLoud::AudioSourceInstance3dData *)inst;
}

AudioCollider AudioCollider_new() {
    return (AudioCollider)new AudioCollider_Derived;
}

void AudioCollider_set_handler(AudioCollider self,
                               float (*handler)(Soloud *, AudioSourceInstance3dData *, int,
                                                void *data),
                               void *data) {
    ((AudioCollider_Derived *)self)->set_handler((AudioCollider_Derived::handler)handler);
    ((AudioCollider_Derived *)self)->set_handler_data(data);
}

void AudioCollider_delete(AudioCollider inst) {
    delete (AudioCollider_Derived *)inst;
}

AudioAttenuator AudioAttenuator_new() {
    return (AudioAttenuator)new AudioAttenuator_Derived;
}

void AudioAttenuator_set_handler(AudioAttenuator self,
                                 float (*handler)(float, float, float, float, void *data),
                                 void *data) {
    ((AudioAttenuator_Derived *)self)->set_handler((AudioAttenuator_Derived::handler)handler);
    ((AudioAttenuator_Derived *)self)->set_handler_data(data);
}

void AudioAttenuator_delete(AudioAttenuator inst) {
    delete (AudioAttenuator_Derived *)inst;
}