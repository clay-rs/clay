#pragma once

#define MAP_RET float3

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

#define MAP_SHAPE_FN_DEF(map_shape, shape, map, mdi, mdf) \
    SHAPE_RET map_shape##_hit(SHAPE_ARGS_DEF) { \
        Ray new_ray = ray; \
        new_ray.start = map##_abs_inv(MAP_ARGS_VB(ray.start, mdi, mdf)); \
        float3 new_dir = map##_rel_inv(MAP_ARGS_VB(ray.dir, mdi, mdf)); \
        float lenf = 1.0f/length(new_dir); \
        new_ray.dir = new_dir*lenf; \
        SHAPE_RET ret = shape##_hit(SHAPE_ARGS_R(new_ray)); \
        if (ret) { \
            *enter *= lenf; \
            *exit *= lenf; \
            *norm = normalize(map##_norm(MAP_ARGS_VB(*norm, mdi, mdf))); \
        } \
        return ret; \
    }
