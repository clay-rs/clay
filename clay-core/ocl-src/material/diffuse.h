#pragma once

#include <clay_core/random.h>
#include "material.h"


__MATERIAL_RET__ diffuse_emit(
    __MATERIAL_ARGS_DEF__
) {
    new_ray->start = pos;
    new_ray->dir = random_hemisphere_cos(norm, seed);
    new_ray->color = ray.color;
    return 1;
}
