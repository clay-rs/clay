#pragma once

#define __SHAPE_RET__ bool

#define __SHAPE_ARGS_DEF__ \
    Ray ray, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *dist, float3 *pos, float3 *norm

#define __SHAPE_ARGS__ \
    ray, ibuf, fbuf, dist, pos, norm

#define __SHAPE_ARGS_B__(di, df) \
    ray, ibuf + (di), fbuf + (df), dist, pos, norm

#define __SHAPE_ARGS_R__(r) \
    (r), ibuf, fbuf, dist, pos, norm
