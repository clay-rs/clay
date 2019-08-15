#pragma once

#include <clay_core/ray.h>


#define TARGET_SIZE_RET bool

#define TARGET_SIZE_ARGS_DEF \
    float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *cos_alpha, float3 *dir

#define TARGET_SIZE_ARGS \
    pos, norm, ibuf, fbuf, cos_alpha, dir

#define TARGET_SIZE_ARGS_B(di, df) \
    pos, norm, ibuf + (di), fbuf + (df), cos_alpha, dir


#define TARGET_SAMPLE_RET bool

#define TARGET_SAMPLE_ARGS_DEF \
    uint *seed, float cos_alpha, float dir, \
    float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float3 *new_dir, float *weight

#define TARGET_SAMPLE_ARGS \
    seed, cos_alpha, dir, pos, norm, ibuf, fbuf, new_dir, weight

#define TARGET_SAMPLE_ARGS_B(di, df) \
    seed, cos_alpha, dir, pos, norm, ibuf + (di), fbuf + (df), new_dir, weight
