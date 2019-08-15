#pragma once

#define SHAPE_HIT_RET bool
#define SHAPE_HIT_RET_BAD false

#define SHAPE_HIT_ARGS_DEF \
    uint *seed, Ray ray, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *enter, float *exit, float3 *norm

#define SHAPE_HIT_ARGS \
    seed, ray, ibuf, fbuf, enter, exit, norm

#define SHAPE_HIT_ARGS_B(di, df) \
    seed, ray, ibuf + (di), fbuf + (df), enter, exit, norm

#define SHAPE_HIT_ARGS_R(r) \
    seed, (r), ibuf, fbuf, enter, exit, norm
