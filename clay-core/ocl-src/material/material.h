#pragma once

#define __MATERIAL_RET__ int

#define __MATERIAL_ARGS_DEF__ \
    Ray r, \
    float3 p, float3 n, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *rr, float3 *glow

#define __MATERIAL_ARGS__ \
    r, p, n, ibuf, fbuf, rr, glow

#define __MATERIAL_ARGS_DBUF__(di, df) \
    r, p, n, ibuf + (di), fbuf + (df), rr, glow
