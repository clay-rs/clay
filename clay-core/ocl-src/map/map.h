#pragma once

#define __MAP_RET__ float3

#define __MAP_ARGS_DEF__ \
    float3 v, \
    __global const int *ibuf, \
    __global const float *fbuf

#define __MAP_ARGS__ \
    v, ibuf, fbuf

#define __MAP_ARGS_DBUF__(di, df) \
    v, ibuf + (di), fbuf + (df)
