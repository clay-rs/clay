#pragma once

#define MATERIAL_EMIT_RET bool
#define MATERIAL_EMIT_RET_BAD false

#define MATERIAL_EMIT_ARGS_DEF \
    uint *seed, Ray ray, \
    float3 pos, float3 norm, \
    bool directed, float3 dir, float size, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *new_ray, float3 *color

#define MATERIAL_EMIT_ARGS \
    seed, ray, pos, norm, directed, dir, size, ibuf, fbuf, new_ray, color

#define MATERIAL_EMIT_ARGS_B(di, df) \
    seed, ray, pos, norm, directed, dir, size, ibuf + (di), fbuf + (df), new_ray, color
