#pragma once

#include <clay_core/matrix.h>
#include "map.h"


__MAP_RET__ affine_rel(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3);
    return matrix3_dot(linear, v);
}

__MAP_RET__ affine_abs(__MAP_ARGS_DEF__) {
    float3 shift = vload3(0, fbuf);
    return affine_rel(__MAP_ARGS__) + shift;
}

__MAP_RET__ affine_rel_inv(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(linear, v);
}

__MAP_RET__ affine_abs_inv(__MAP_ARGS_DEF__) {
    float3 shift = vload3(0, fbuf);
    return affine_rel_inv(__MAP_ARGS_V__(v - shift));
}

__MAP_RET__ affine_norm(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(matrix3_transpose(linear), v);
}
