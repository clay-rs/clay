#pragma once

#include "material.h"


#define __COLORED_MATERIAL_FN_DEF__(color_material_fn, material_fn, mdi, mdf) \
    __MATERIAL_RET__ color_material_fn(__MATERIAL_ARGS_DEF__) { \
        ray.color *= vload3(0, fbuf + mdf); \
        return material_fn(__MATERIAL_ARGS__); \
    }
