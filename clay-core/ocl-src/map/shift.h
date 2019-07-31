#pragma once

#include <map/map.h>

typedef float3 Shift;

Shift shift_load(__global const int *ibuf, __global const float *fbuf) {
    return vload3(0, fbuf);
}

void shift_store(Shift s, __global int *ibuf, __global float *fbuf) {
    vstore3(s, 0, fbuf);
}

__MAP_RET__ shift_abs(__MAP_ARGS_DEF__) {
    return v + shift_load(ibuf, fbuf);
}

__MAP_RET__ shift_rel(__MAP_ARGS_DEF__) {
    return v;
}

__MAP_RET__ shift_abs_inv(__MAP_ARGS_DEF__) {
    return v - shift_load(ibuf, fbuf);
}

__MAP_RET__ shift_rel_inv(__MAP_ARGS_DEF__) {
    return v;
}

__MAP_RET__ shift_norm_inv(__MAP_ARGS_DEF__) {
    return v;
}
