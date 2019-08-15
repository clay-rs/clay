#pragma once

#include <clay_core/random.h>
#include <clay_core/linalg.h>
#include <clay_core/matrix.h>
#include "material.h"


MATERIAL_EMIT_RET diffuse_emit(
    MATERIAL_EMIT_ARGS_DEF
) {
    new_ray->start = pos;

    float3 rand_dir = random_hemisphere_cosine(seed);
    matrix3 basis = { .z = norm };
    complement(basis.z, &basis.x, &basis.y);
    new_ray->dir = matrix3_dot(matrix3_transpose(basis), rand_dir);

    new_ray->color = ray.color;
    new_ray->type = RAY_DIFFUSE;

    return true;
}
