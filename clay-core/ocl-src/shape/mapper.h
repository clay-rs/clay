#pragma once

#include <clay_core/map/map.h>
#include <clay_core/shape/shape.h>


#define MAP_SHAPE_FN_DEF(map_shape, shape, map, sdi, sdf) \
    SHAPE_HIT_RET map_shape##_hit(SHAPE_HIT_ARGS_DEF) { \
        Ray new_ray = ray; \
        new_ray.start = map##_abs_inv(MAP_ARGS_VB(ray.start, sdi, sdf)); \
        float3 new_dir = map##_rel_inv(MAP_ARGS_VB(ray.dir, sdi, sdf)); \
        float lenf = 1.0f/length(new_dir); \
        new_ray.dir = new_dir*lenf; \
        SHAPE_HIT_RET ret = shape##_hit(SHAPE_HIT_ARGS_R(new_ray)); \
        if (ret) { \
            *enter *= lenf; \
            *exit *= lenf; \
            *norm = normalize(map##_norm(MAP_ARGS_VB(*norm, sdi, sdf))); \
        } \
        return ret; \
    }
