#pragma once

#include <clay_core/ray.h>
#include "shape.h"


SHAPE_HIT_RET unitsphere_hit(
    SHAPE_HIT_ARGS_DEF
) {
    // t^2 - 2*b*t + c = 0
    float b = -dot(ray.dir, ray.start);
    float c = dot(ray.start, ray.start) - 1.0f;
    float d = b*b - c;
    if (d < 0.0f) {
        return false;
    }
    d = sqrt(d);
    float e = b - d;
    if (e < 0.0f) {
        return false;
    }
    *enter = e;
    *exit = b + d;
    *norm = ray.start + ray.dir*e;
    return true;
}

/*
TARGET_SIZE_RET sphere_target_size(
    TARGET_SIZE_ARGS_DEF
) {
    float rad = fbuf[0];
    float3 tpos = vload3(0, fbuf + 1);

    float3 dir = tpos - pos;
    float len = length(dir);
    dir /= len;
    
    float sin_alpha = rad/len;
    if (
        (sin_alpha > threshold) ||
        (sin_alpha + dot(dir, norm) < 0.0f)
    ) {
        return -1;
    }
    float cos_alpha = sqrt(1.0f - sin_alpha*sin_alpha);

    return cos_alpha;
}

TARGET_SAMPLE_RET sphere_target_sample(
    TARGET_SAMPLE_ARGS_DEF
) {
    float rad = fbuf[0];
    float3 pos = vload3(0, fbuf + 1);

    float3 dir = pos - ray.start;
    float len = length(dir);
    dir /= len;
    
    float sin_alpha = rad/len;
    if (
        (sin_alpha > threshold) ||
        (sin_alpha + dot(dir, norm) < 0.0f)
    ) {
        return -1;
    }
    float cos_alpha = sqrt(1.0f - sin_alpha*sin_alpha);
    *weight = 1.0f - cos_alpha;

    new_ray->start = ray.start;

    float3 rand_dir = random_sphere_cap(seed, cos_alpha);
    matrix3 basis = { .z = dir };
    complement(basis.z, &basis.x, &basis.y);
    new_ray->dir = matrix3_dot(matrix3_transpose(basis), rand_dir);

    float asc = dot(new_ray->dir, norm);
    if (asc < 0.0f) {
        return 0;
    }

    new_ray->color = *weight*2.0f*asc*ray.color;
    new_ray->type = RAY_ATTRACT;

    return 1;
}
*/
