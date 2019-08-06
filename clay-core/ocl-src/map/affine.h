#pragma once

#include <clay_core/matrix.h>
#include "map.h"


MAP_RET affine_rel(MAP_ARGS_DEF) {
    matrix3 linear = matrix3_load(fbuf + 3);
    return matrix3_dot(linear, v);
}

MAP_RET affine_abs(MAP_ARGS_DEF) {
    float3 shift = vload3(0, fbuf);
    return affine_rel(MAP_ARGS) + shift;
}

MAP_RET affine_rel_inv(MAP_ARGS_DEF) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(linear, v);
}

MAP_RET affine_abs_inv(MAP_ARGS_DEF) {
    float3 shift = vload3(0, fbuf);
    return affine_rel_inv(MAP_ARGS_V(v - shift));
}

MAP_RET affine_norm(MAP_ARGS_DEF) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(matrix3_transpose(linear), v);
}
