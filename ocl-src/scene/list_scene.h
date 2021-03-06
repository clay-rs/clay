#pragma once

#include <clay_core/random.h>


#define SCENE_ARGS_DEF \
    __global const int *object_buffer_int, \
    __global const float *object_buffer_float, \
    int objects_count, \
    int max_depth, \
    \
    BACKGROUND_ARGS_DEF

#define SCENE_ARGS \
    object_buffer_int, \
    object_buffer_float, \
    objects_count, \
    max_depth, \
    \
    BACKGROUND_ARGS


bool scene_trace(
    uint *seed,
    Ray ray,
    Ray *new_ray,
    float3 *color,
    SCENE_ARGS_DEF
) {
    int hit_idx = -1;
    float hit_enter = INFINITY;
    float hit_exit = 0.0f;
    float3 hit_norm;

    int i = 0;
    for (i = 0; i < objects_count; ++i) {
        float enter, exit;
        float3 norm;

        if (ray.origin == i) {
            continue;
        }
        
        __global const int *ibuf = object_buffer_int + OBJECT_SIZE_INT*i;
        __global const float *fbuf = object_buffer_float + OBJECT_SIZE_FLOAT*i;
        if (__object_hit(seed, ray, ibuf, fbuf, &enter, &exit, &norm)) {
            if (enter < hit_enter) {
                hit_enter = enter;
                hit_exit = exit;
                hit_norm = norm;
                hit_idx = i;
            }
        }
    }
    
    if (hit_idx >= 0) {
        float3 hit_pos = ray.start + ray.dir*hit_enter;

        __global const int *ibuf = object_buffer_int + OBJECT_SIZE_INT*hit_idx;
        __global const float *fbuf = object_buffer_float + OBJECT_SIZE_FLOAT*hit_idx;
        if(__object_bounce(
            seed, ray, hit_pos, hit_norm,
            false, (float3)(0.0f), 0.0f,
            ibuf, fbuf, new_ray, color
        )) {
            new_ray->origin = hit_idx;
            return true;
        }
        return false;
    }

    // Background
    *color += __background(ray, BACKGROUND_ARGS);
    return false;
}

float3 __scene_trace(
    uint *seed,
    Ray ray,
    SCENE_ARGS_DEF
) {
    float3 color = (float3)(0.0f);
    int i = 0;
    Ray current_ray = ray;
    for (i = 0; i < max_depth; ++i) {
        Ray next_ray = ray_new();
        bool bounce = scene_trace(seed, current_ray, &next_ray, &color, SCENE_ARGS);
        if (!bounce) {
            break;
        }
        current_ray = next_ray;
    }
    return color;
}
