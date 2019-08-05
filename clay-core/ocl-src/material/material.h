#pragma once

#define __MATERIAL_RET__ int

#define __MATERIAL_ARGS_DEF__ \
    uint *seed, \
    Ray ray, \
    float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *new_ray, float3 *color

#define __MATERIAL_ARGS__ \
    seed, ray, pos, norm, ibuf, fbuf, new_ray, color

#define __MATERIAL_ARGS_B__(di, df) \
    seed, ray, pos, norm, ibuf + (di), fbuf + (df), new_ray, color
