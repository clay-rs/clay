#pragma once

#include <clay_core/random.h>
#include <clay_core/linalg.h>
#include <clay_core/matrix.h>
#include "material.h"


__MATERIAL_RET__ diffuse_emit(
    __MATERIAL_ARGS_DEF__
) {
    new_ray->start = pos;
    float3 rhc = random_hemisphere_cosine(seed);
    matrix3 basis = { .z = norm };
    complement(basis.z, &basis.x, &basis.y);
    new_ray->dir = matrix3_dot(matrix3_transpose(basis), rhc);
    new_ray->color = ray.color;
    return 1;
}
