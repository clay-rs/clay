#pragma once

#define MAP_RET float3
#define MAP_RET_BAD (float3)(0.0f)

#define MAP_ARGS_DEF \
    float3 v, \
    __global const int *ibuf, \
    __global const float *fbuf

#define MAP_ARGS \
    v, ibuf, fbuf

#define MAP_ARGS_VB(v, di, df) \
    (v), ibuf + (di), fbuf + (df)

#define MAP_ARGS_V(v) \
    MAP_ARGS_VB(v, 0, 0)

#define MAP_ARGS_B(di, df) \
    MAP_ARGS_VB(v, di, df)
