#pragma once

#include "soloud/include/soloud_c.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void AudioSourceInstance3dData;

AudioSourceInstance3dData *AudioSourceInstance3dData_new(AudioSource *as);

void AudioSourceInstance3dData_delete(AudioSourceInstance3dData *inst);

AudioCollider AudioCollider_new();

void AudioCollider_set_handler(AudioCollider self,
                               float (*handler)(Soloud *, AudioSourceInstance3dData *, int, void *),
                               void *data);

void AudioCollider_delete(AudioCollider);

AudioAttenuator AudioAttenuator_new();

void AudioAttenuator_set_handler(AudioAttenuator self,
                                 float (*handler)(float, float, float, float, void *), void *data);

void AudioAttenuator_delete(AudioAttenuator);

#ifdef __cplusplus
}
#endif