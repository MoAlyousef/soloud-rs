#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void *AudioSourceInstance3dData_new(void *as);

void AudioSourceInstance3dData_delete(void *inst);

void *AudioCollider_new();

void AudioCollider_set_handler(void *self, float (*handler)(void *, void *, int, void *), void *data);

void AudioCollider_delete(void *);

void *AudioAttenuator_new();

void AudioAttenuator_set_handler(void *self, float (*handler)(float, float, float, float, void *), void *data);

void AudioAttenuator_delete(void *);

#ifdef __cplusplus
}
#endif