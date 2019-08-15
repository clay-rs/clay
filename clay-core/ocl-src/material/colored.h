#pragma once

#include "material.h"


#define COLORED_MATERIAL_FN_DEF(colored_material, material, mdi, mdf) \
    MATERIAL_EMIT_RET colored_material##_emit(MATERIAL_EMIT_ARGS_DEF) { \
        ray.color *= vload3(0, fbuf + mdf); \
        return material##_emit(MATERIAL_EMIT_ARGS); \
    }
