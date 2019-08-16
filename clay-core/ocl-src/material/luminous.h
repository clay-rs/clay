#pragma once

#include "material.h"


MATERIAL_EMIT_RET luminous_emit(
    MATERIAL_EMIT_ARGS_DEF
) {
    *color += ray.color;
    return false;
}
