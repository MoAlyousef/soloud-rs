#include "soloud.h"
#include "soloud_audiosource.h"

#include "soloud_derives.h"

using namespace SoLoud;

struct AudioCollider_Derived : public AudioCollider {
    void *ev_data_ = NULL;   
    typedef float (*handler)(void *, void *, int, void *data); 
    handler inner_handler = NULL;
    void set_handler(handler h) {                                                              
        inner_handler = h;                                                                     
    }                                                                                          
    void set_handler_data(void *data) {                                                        
        ev_data_ = data;                                                                       
    }  
    virtual float collide(Soloud *aSoloud, AudioSourceInstance3dData *aAudioInstance3dData,	int aUserData) override {
        if (!ev_data_ || !inner_handler) return 0;
        return inner_handler(aSoloud, aAudioInstance3dData, aUserData, ev_data_);
    }
};

struct AudioAttenuator_Derived : public AudioAttenuator {
    void *ev_data_ = NULL;   
    typedef float (*handler)(float, float, float, float, void *data); 
    handler inner_handler = NULL;
    void set_handler(handler h) {                                                              
        inner_handler = h;                                                                     
    }                                                                                          
    void set_handler_data(void *data) {                                                        
        ev_data_ = data;                                                                       
    }  
    virtual float attenuate(float aDistance, float aMinDistance, float aMaxDistance, float aRolloffFactor) override {
        if (!ev_data_ || !inner_handler) return 0;
        return inner_handler(aDistance, aMinDistance, aMaxDistance, aRolloffFactor, ev_data_);
    }
};

void *AudioSourceInstance3dData_new(void *as) {
    auto temp = new AudioSourceInstance3dData();
    temp->init(*(AudioSource *)as);
    return temp;
}

void AudioSourceInstance3dData_delete(void *inst) {
    delete (AudioSourceInstance3dData *)inst;
}

void *AudioCollider_new() {
    return new AudioCollider_Derived();
}

void AudioCollider_set_handler(void *self, float (*handler)(void *, void *, int, void *data), void *data) {
    ((AudioCollider_Derived *)self)->set_handler(handler);
    ((AudioCollider_Derived *)self)->set_handler_data(data);
}

void AudioCollider_delete(void *inst) {
    delete (AudioCollider_Derived *)inst;
}

void *AudioAttenuator_new() {
    return new AudioAttenuator_Derived();
}

void AudioAttenuator_set_handler(void *self, float (*handler)(float, float, float, float, void *data), void *data) {
    ((AudioAttenuator_Derived *)self)->set_handler(handler);
    ((AudioAttenuator_Derived *)self)->set_handler_data(data);
}

void AudioAttenuator_delete(void *inst) {
    delete (AudioAttenuator_Derived *)inst;
}