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

#define ATTRACT_THRESHOLD 0.1f


int scene_trace(
    uint *seed,
    Ray ray,
    Ray *new_rays,
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
        if (
            (ray.type == RAY_DIFFUSE && ray.target == hit_idx) ||
            (ray.type == RAY_ATTRACT && ray.target != hit_idx)
        ) {
            return 0;
        }

        float3 hit_pos = ray.start + ray.dir*hit_enter;

        __global const int *ibuf = objects_int + object_size_int*hit_idx;
        __global const float *fbuf = objects_float + object_size_float*hit_idx;
        int num_rays = __object_emit(
            seed, ray, hit_pos, hit_norm,
            ibuf, fbuf, &new_rays[0], color
        );
        if (num_rays == 1) {
            int rays_count = 1;
            new_rays[0].origin = hit_idx;

            // Attraction
            if (new_rays[0].type == RAY_DIFFUSE) {
                int attract_idx = (int)(random_uniform(seed)*attractors_count);
                __global const int *aibuf = attractors_int + attractor_size_int*attract_idx;
                __global const float *afbuf = attractors_float + attractor_size_float*attract_idx;

                int target = aibuf[0];
                Ray attract_ray = ray_new();
                float weight = 0.0f;
                int ret = __attract(
                    seed, new_rays[0], hit_norm, ATTRACT_THRESHOLD,
                    aibuf, afbuf, &attract_ray, &weight
                );
                if (ret >= 0) {
                    new_rays[0].target = target;
                    new_rays[0].color *= (1.0f - weight);
                    if (ret == 1) {
                        attract_ray.origin = hit_idx;
                        attract_ray.target = target;
                        new_rays[1] = attract_ray;
                        rays_count += 1;
                    }
                }
            }

            return rays_count;
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
        Ray next_rays[2] = { ray_new() };
        int num_rays = scene_trace(seed, current_ray, next_rays, &color, SCENE_ARGS);
        if (num_rays == 0) {
            break;
        }
        if (num_rays == 2) {
            scene_trace(seed, next_rays[1], NULL, &color, SCENE_ARGS);
        }
        current_ray = next_rays[0];
    }
    return color;
}
