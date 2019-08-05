#pragma once

#define __SCENE_ARGS_DEF__ \
    __global const int *objects_int, \
    __global const float *objects_float, \
    int size_int, \
    int size_float, \
    int objects_count

#define __SCENE_ARGS__ \
    objects_int, \
    objects_float, \
    size_int, \
    size_float, \
    objects_count

#define MAX_DEPTH 8

int scene_trace(
    uint *seed,
    Ray ray,
    Ray *new_ray,
    float3 *color,
    __SCENE_ARGS_DEF__
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
        
        __global const int *ibuf = objects_int + size_int*i;
        __global const float *fbuf = objects_float + size_float*i;
        if (__object_hit__(seed, ray, ibuf, fbuf, &enter, &exit, &norm)) {
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

        __global const int *ibuf = objects_int + size_int*hit_idx;
        __global const float *fbuf = objects_float + size_float*hit_idx;
        int num_rays = __object_emit__(seed, ray, hit_pos, hit_norm, ibuf, fbuf, new_ray, color);
        if (num_rays > 0) {
            new_ray->origin = hit_idx;
            return 1;
        } else {
            return 0;
        }
    } else {
        float z = 0.5f*(ray.dir.z + 1.0f);
        *color += ray.color*(float3)(z, z, z);
        return 0;
    }
}

float3 __scene_trace__(
    uint *seed,
    Ray ray,
    __SCENE_ARGS_DEF__
) {
    float3 color = (float3)(0.0f);
    int i = 0;
    Ray current_ray = ray;
    for (i = 0; i < MAX_DEPTH; ++i) {
        Ray next_ray;
        int num_rays = scene_trace(seed, current_ray, &next_ray, &color, __SCENE_ARGS__);
        if (num_rays == 0) {
            break;
        }
        current_ray = next_ray;
    }
    return color;
}
