#pragma once

#include <clay_core/random.h>


#define SCENE_ARGS_DEF \
    __global const int *object_buffer_int, \
    __global const float *object_buffer_float, \
    int objects_count, \
    \
    __global const int *target_buffer_int, \
    __global const float *target_buffer_float, \
    int targets_count, \
    \
    int max_depth, \
    \
    BACKGROUND_ARGS_DEF

#define SCENE_ARGS \
    object_buffer_int, \
    object_buffer_float, \
    objects_count, \
    \
    target_buffer_int, \
    target_buffer_float, \
    targets_count, \
    \
    max_depth, \
    \
    BACKGROUND_ARGS

#define TARGET_THRESHOLD 0.1f

#define OBJ_DI 1
#define OBJ_DF 0
#define TAR_DI 1
#define TAR_DF 1


bool scene_trace(
    uint *seed,
    Ray ray,
    Ray *new_ray,
    float3 *color,
    SCENE_ARGS_DEF
) {
    int hit_idx = -1;
    int tar_idx = -1;
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
        if (__object_hit(
            seed, ray,
            ibuf + OBJ_DI, fbuf + OBJ_DF,
            &enter, &exit, &norm
        )) {
            if (enter < hit_enter) {
                hit_enter = enter;
                hit_exit = exit;
                hit_norm = norm;
                hit_idx = i;
                tar_idx = ibuf[0];
            }
        }
    }
    
    if (hit_idx >= 0) {
        if (ray.history & RAY_TARGETED) {
            if (ray.target != hit_idx) {
                return false;
            }
        } else if (ray.history & RAY_DIFFUSE) {
            if (ray.target != tar_idx) {
                return false;
            }
        }

        float3 hit_pos = ray.start + ray.dir*hit_enter;

        // Sample target
        int target = -1;
        bool directed = false;
        float target_size = 0.0f;
        float3 target_dir = (float3)(0.0f);
        if (random_uniform(seed) > 0.5f) {
            int target_idx = floor(random_uniform(seed)*targets_count);
            __global const int *tibuf = target_buffer_int + TARGET_SIZE_INT*target_idx;
            __global const float *tfbuf = target_buffer_float + TARGET_SIZE_FLOAT*target_idx;

            //float brightness = tfbuf[0];
            target = tibuf[0];
            target_size = __target_sample(
                seed, hit_pos,
                tibuf + TAR_DI, tfbuf + TAR_DF,
                &target_dir
            );
            directed = true;
        }

        // Bounce from material
        __global const int *ibuf = object_buffer_int + OBJECT_SIZE_INT*hit_idx;
        __global const float *fbuf = object_buffer_float + OBJECT_SIZE_FLOAT*hit_idx;
        bool bounce = __object_bounce(
            seed, ray, hit_pos, hit_norm,
            directed, target_dir, target_size,
            ibuf + OBJ_DI, fbuf + OBJ_DF, new_ray, color
        );
        if (bounce && !(ray.history & RAY_TARGETED)) {
            new_ray->origin = hit_idx;
            if (directed) {
                new_ray->target = target;
                new_ray->history |= RAY_TARGETED;
                new_ray->color *= 2.0f*targets_count; // reverse probability of specific target sampling
            } else {
                new_ray->color *= 2.0f; // reverse probability of not sampling any target
            }
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
