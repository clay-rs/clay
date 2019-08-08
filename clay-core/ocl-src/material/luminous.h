#pragma once

#include "material.h"


MATERIAL_RET luminous_emit(
    MATERIAL_ARGS_DEF
) {
	*color += ray.color;
    return 0;
}
