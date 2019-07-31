#pragma once

#include <map/map.h>

#define __MAP_RET__ float3

#define __MAP_ARGS_DEF__ \
    float3 v, \
    __global const int *ibuf, \
    __global const float *fbuf

#define __MAP_ARGS__ \
    v, ibuf, fbuf

#define __MAP_ARGS_VB__(v, di, df) \
    (v), ibuf + (di), fbuf + (df)

#define __MAP_ARGS_V__(v) \
    __MAP_ARGS_VB__(v, 0, 0)

#define __MAP_ARGS_B__(di, df) \
    __MAP_ARGS_VB__(v, di, df)

#define MAP_SHAPE_FN_DEF(map_shape_fn, shape_fn, map_pref, mdi, mdf) \
    __SHAPE_RET__ map_shape_fn(__SHAPE_ARGS_DEF__) { \
        Ray new_ray = ray; \
        new_ray.start = map_pref##_abs_inv(__MAP_ARGS_VB__(ray.start, mdi, mdf)); \
        new_ray.dir = normalize(map_pref##_rel_inv(__MAP_ARGS_VB__(ray.dir, mdi, mdf))); \
        __SHAPE_RET__ ret = shape_fn(__SHAPE_ARGS_R__(new_ray)); \
        if (ret) { \
            *pos = map_pref##_abs(__MAP_ARGS_VB__(*pos, mdi, mdf)); \
            *norm = normalize(map_pref##_norm(__MAP_ARGS_VB__(*norm, mdi, mdf))); \
            *dist = dot(*pos - ray.start, ray.dir); \
        } \
        return ret; \
    }
