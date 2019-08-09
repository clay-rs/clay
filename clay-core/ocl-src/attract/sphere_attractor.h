#pragma once

#include <clay_core/ray.h>
#include <clay_core/random.h>

#include "attractor.h"


ATTRACTOR_RET sphere_attract(
    ATTRACTOR_ARGS_DEF
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
