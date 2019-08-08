#pragma once

#include <clay_core/random.h>


#define SCENE_ARGS_DEF \
    __global const int *objects_int, \
    __global const float *objects_float, \
    __global const int *attractors_int, \
    __global const float *attractors_float, \
    int object_size_int, \
    int object_size_float, \
    int attractor_size_int, \
    int attractor_size_float, \
    int objects_count, \
    int attractors_count

#define SCENE_ARGS \
    objects_int, \
    objects_float, \
    attractors_int, \
    attractors_float, \
    object_size_int, \
    object_size_float, \
    attractor_size_int, \
    attractor_size_float, \
    objects_count, \
    attractors_count

#define MAX_DEPTH 4

int scene_trace(
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
        
        __global const int *ibuf = objects_int + object_size_int*i;
        __global const float *fbuf = objects_float + object_size_float*i;
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

        __global const int *ibuf = objects_int + object_size_int*hit_idx;
        __global const float *fbuf = objects_float + object_size_float*hit_idx;
        int num_rays = __object_emit(seed, ray, hit_pos, hit_norm, ibuf, fbuf, new_ray, color);
        if (num_rays > 0) {
            new_ray->origin = hit_idx;

            // attract
            int attract_idx = (int)(random_uniform(seed)*attractors_count);
            __global const int *ibuf = attractors_int + attractor_size_int*attract_idx;
            __global const float *fbuf = attractors_float + attractor_size_float*attract_idx;
            

            return 1;
        } else {
            return 0;
        }
    } else {
        float z = 0.5f*(ray.dir.z + 1.0f);
        *color += ray.color*z*(float3)(0.2, 0.2, 0.4);
        return 0;
    }
}

float3 __scene_trace(
    uint *seed,
    Ray ray,
    SCENE_ARGS_DEF
) {
    float3 color = (float3)(0.0f);
    int i = 0;
    Ray current_ray = ray;
    for (i = 0; i < MAX_DEPTH; ++i) {
        Ray next_ray;
        int num_rays = scene_trace(seed, current_ray, &next_ray, &color, SCENE_ARGS);
        if (num_rays == 0) {
            break;
        }
        current_ray = next_ray;
    }
    return color;
}
