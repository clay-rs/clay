#pragma once

#include <clay_core/random.h>
#include <clay_core/linalg.h>
#include <clay_core/matrix.h>
#include <clay_core/material/material.h>


#define DIFFUSE_COSINE

MATERIAL_BOUNCE_RET diffuse_bounce(
    MATERIAL_BOUNCE_ARGS_DEF
) {
    new_ray->start = pos;

    if (!directed) {
        float3 rand_dir =
        #ifdef DIFFUSE_COSINE
            random_hemisphere_cosine(seed);
        #else
            random_hemisphere(seed);
        #endif // DIFFUSE_COSINE

        matrix3 basis = { .z = norm };
        complement(basis.z, &basis.x, &basis.y);
        new_ray->dir = matrix3_dot(matrix3_transpose(basis), rand_dir);
        new_ray->color = 
        #ifndef DIFFUSE_COSINE
            2.0f*rand_dir.z*
        #endif // DIFFUSE_COSINE
            ray.color;
    } else {
        float cos_theta = dot(dir, norm);
        if (cos_theta < 0.0f) {
            return false;
        }
        new_ray->dir = dir;
        new_ray->color = 2.0f*cos_theta*size*ray.color;
    }

    new_ray->history = ray.history | RAY_DIFFUSE;

    return true;
}
