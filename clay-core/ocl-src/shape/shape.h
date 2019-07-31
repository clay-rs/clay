#pragma once

#define __SHAPE_RET__ bool

#define __SHAPE_ARGS_DEF__ \
    Ray r, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *d, float3 *p, float3 *n

#define __SHAPE_ARGS__ \
    r, ibuf, fbuf, d, p, n

#define __SHAPE_ARGS_DBUF__(di, df) \
    r, ibuf + (di), fbuf + (df), d, p, n
