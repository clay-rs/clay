#pragma once

#include <clay_core/ray.h>


#define ATTRACTOR_RET int

// If angular radius is higher than threshold then
// attraction should not occur.
#define ATTRACTOR_ARGS_DEF \
    uint *seed, Ray ray, float3 norm, \
    float threshold, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *new_ray, float *weight

#define ATTRACTOR_ARGS \
    seed, ray, norm, threshold, ibuf, fbuf, new_ray, weight
