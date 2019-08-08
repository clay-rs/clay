#pragma once

#include <clay_core/ray.h>


#define ATTRACT_RET Ray

#define ATTRACT_ARGS_DEF \
    uint *seed, float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf

#define ATTRACT_ARGS \
    seed, pos, norm, ibuf, fbuf
