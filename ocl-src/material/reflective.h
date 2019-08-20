#pragma once

#include <clay_core/material/material.h>


MATERIAL_BOUNCE_RET reflective_bounce(
    MATERIAL_BOUNCE_ARGS_DEF
) {
    if (directed) {
        return false;
    }
    new_ray->start = pos;
    new_ray->dir = ray.dir - 2.0f*norm*dot(norm, ray.dir);
    new_ray->color = ray.color;
    new_ray->history = ray.history;
    return true;
}
