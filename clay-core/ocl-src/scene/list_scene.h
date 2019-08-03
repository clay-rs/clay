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


float3 scene_trace(
    Ray ray,
    int depth,
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
        if (__object_hit__(ray, ibuf, fbuf, &enter, &exit, &norm)) {
            if (enter < hit_enter) {
                hit_enter = enter;
                hit_exit = exit;
                hit_norm = norm;
                hit_idx = i;
            }
        }
    }
    
    float3 color = (float3)(0.0f);
    if (hit_idx >= 0 && depth < 4) {
        Ray new_ray;
        float3 glow = (float3)(0.0f);
        float3 hit_pos = ray.start + ray.dir*hit_enter;

        __global const int *ibuf = objects_int + size_int*hit_idx;
        __global const float *fbuf = objects_float + size_float*hit_idx;
        int num_rays = __object_emit__(ray, hit_pos, hit_norm, ibuf, fbuf, &new_ray, &glow);
        new_ray.origin = hit_idx;
        color += glow;
        if (num_rays > 0) {
            color += scene_trace(new_ray, depth + 1, __SCENE_ARGS__);
        }
    } else {
        float z = 0.5f*(ray.dir.z + 1.0f);
        color = ray.color*(float3)(z, z, z);
    }
    return color;
}

float3 __scene_trace__(
    Ray ray,
    __SCENE_ARGS_DEF__
) {
    return scene_trace(ray, 0, __SCENE_ARGS__);
}
