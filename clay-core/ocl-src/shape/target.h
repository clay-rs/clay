#pragma once

#include <clay_core/ray.h>


// returns angular size of the target
#define TARGET_SIZE_RET float

#define TARGET_SIZE_ARGS_DEF \
    float3 pos, \
    __global const int *ibuf, \
    __global const float *fbuf

#define TARGET_SIZE_ARGS \
    pos, ibuf, fbuf

#define TARGET_SIZE_ARGS_B(di, df) \
    pos, ibuf + (di), fbuf + (df)


// returns sample direction
#define TARGET_SAMPLE_RET float3

#define TARGET_SAMPLE_ARGS_DEF \
    uint *seed, float3 pos, float size, \
    __global const int *ibuf, \
    __global const float *fbuf

#define TARGET_SAMPLE_ARGS \
    seed, pos, size, ibuf, fbuf

#define TARGET_SAMPLE_ARGS_B(di, df) \
    seed, pos, size, ibuf + (di), fbuf + (df)
