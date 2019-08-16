#pragma once

#include "material.h"


#define COLORED_MATERIAL_FN_DEF(colored_material, material, mdi, mdf) \
    MATERIAL_BOUNCE_RET colored_material##_bounce(MATERIAL_BOUNCE_ARGS_DEF) { \
        ray.color *= vload3(0, fbuf + mdf); \
        return material##_bounce(MATERIAL_BOUNCE_ARGS); \
    }
