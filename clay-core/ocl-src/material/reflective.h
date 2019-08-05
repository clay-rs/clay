#pragma once

#include "material.h"

__MATERIAL_RET__ reflective_emit(
    __MATERIAL_ARGS_DEF__
) {
    new_ray->start = pos;
    new_ray->dir = ray.dir - 2.0f*norm*dot(norm, ray.dir);
    new_ray->color = ray.color;
    return 1;
}
