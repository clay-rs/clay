#pragma once

#include <clay_core/ray.h>

// returns angular size of the target
#define TARGET_SAMPLE_RET float

#define TARGET_SAMPLE_ARGS_DEF \
    uint *seed, float3 pos, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float3 *dir // sample direction

#define TARGET_SAMPLE_ARGS \
    seed, pos, ibuf, fbuf, dir

#define TARGET_SAMPLE_ARGS_B(di, df) \
    seed, pos, ibuf + (di), fbuf + (df), dir
