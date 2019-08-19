#pragma once

#include <clay_core/material/material.h>


MATERIAL_BOUNCE_RET luminous_bounce(
    MATERIAL_BOUNCE_ARGS_DEF
) {
    *color += ray.color;
    return false;
}
