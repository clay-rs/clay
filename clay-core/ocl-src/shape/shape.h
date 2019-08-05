#pragma once

#define __SHAPE_RET__ bool

#define __SHAPE_ARGS_DEF__ \
    uint *seed, Ray ray, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *enter, float *exit, float3 *norm

#define __SHAPE_ARGS__ \
    seed, ray, ibuf, fbuf, enter, exit, norm

#define __SHAPE_ARGS_B__(di, df) \
    seed, ray, ibuf + (di), fbuf + (df), enter, exit, norm

#define __SHAPE_ARGS_R__(r) \
    seed, (r), ibuf, fbuf, enter, exit, norm
