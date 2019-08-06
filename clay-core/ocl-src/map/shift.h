#pragma once

#include "map.h"

typedef float3 Shift;

Shift shift_load(__global const int *ibuf, __global const float *fbuf) {
    return vload3(0, fbuf);
}

MAP_RET shift_rel(MAP_ARGS_DEF) {
    return v;
}

MAP_RET shift_abs(MAP_ARGS_DEF) {
    return v + shift_load(ibuf, fbuf);
}

MAP_RET shift_rel_inv(MAP_ARGS_DEF) {
    return v;
}

MAP_RET shift_abs_inv(MAP_ARGS_DEF) {
    return v - shift_load(ibuf, fbuf);
}

MAP_RET shift_norm(MAP_ARGS_DEF) {
    return v;
}
