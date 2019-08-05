#pragma once

#include "shape.h"


float _cube_hit_nearest(float3 near, float3 *norm) {
    bool xy = near.x > near.y;
    bool yz = near.y > near.z;
    bool xz = near.x > near.z;
    float dist = 0.0;
    if (xy && xz) {
        dist = near.x;
        norm->x = 1.0f;
    } else if (yz) {
        dist = near.y;
        norm->y = 1.0f;
    } else {
        dist = near.z;
        norm->z = 1.0f;
    }
    return dist;
}


__SHAPE_RET__ cube_hit(
    __SHAPE_ARGS_DEF__
) {
    const float3 cmax = (float3)(1.0f);
    const float3 cmin = (float3)(-1.0f);

    float3 inv_dir = 1.0f/ray.dir;

    float3 vmin = (cmin - ray.start)*inv_dir;
    float3 vmax = (cmax - ray.start)*inv_dir;

    float3 near = min(vmin, vmax);
    float3 far = max(vmin, vmax);

    float3 norm_in = (float3)(0.0f);
    float dist_in = _cube_hit_nearest(near, &norm_in);
    norm_in *= -sign(ray.dir);

    float3 norm_out = (float3)(0.0f);
    float dist_out = -_cube_hit_nearest(-far, &norm_out);
    norm_out *= sign(ray.dir);

    if (dist_in < 0.0f || dist_in > dist_out) {
        return false;
    }

    *enter = dist_in;
    *exit = dist_out;
    *norm = norm_in;
    return true;
}
