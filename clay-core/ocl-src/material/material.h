#pragma once

#define __MATERIAL_RET__ int

#define __MATERIAL_ARGS_DEF__ \
    Ray ray, \
    float3 pos, float3 norm, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *new_ray, float3 *color

#define __MATERIAL_ARGS__ \
    ray, pos, norm, ibuf, fbuf, new_ray, color

#define __MATERIAL_ARGS_B__(di, df) \
    ray, pos, norm, ibuf + (di), fbuf + (df), new_ray, color
