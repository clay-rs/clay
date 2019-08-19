#pragma once

#include <clay_core/matrix.h>
#include <clay_core/map/map.h>


MAP_RET linear_rel(MAP_ARGS_DEF) {
    matrix3 linear = matrix3_load(fbuf);
    return matrix3_dot(linear, v);
}

MAP_RET linear_abs(MAP_ARGS_DEF) {
    return linear_rel(MAP_ARGS);
}

MAP_RET linear_rel_inv(MAP_ARGS_DEF) {
    matrix3 inverse = matrix3_load(fbuf + 9);
    return matrix3_dot(inverse, v);
}

MAP_RET linear_abs_inv(MAP_ARGS_DEF) {
    return linear_rel_inv(MAP_ARGS);
}

MAP_RET linear_norm(MAP_ARGS_DEF) {
    matrix3 inverse = matrix3_load(fbuf + 9);
    return matrix3_dot(matrix3_transpose(inverse), v);
}
