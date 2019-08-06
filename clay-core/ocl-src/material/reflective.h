#pragma once

#include "material.h"


MATERIAL_RET reflective_emit(
    MATERIAL_ARGS_DEF
) {
    new_ray->start = pos;
    new_ray->dir = ray.dir - 2.0f*norm*dot(norm, ray.dir);
    new_ray->color = ray.color;
    return 1;
}
