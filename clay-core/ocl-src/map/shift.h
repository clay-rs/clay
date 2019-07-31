#pragma once

typedef float3 Shift;

Shift shift_load(__global const int *ibuf, __global const float *fbuf) {
    return vload3(0, fbuf);
}

void shift_store(Shift s, __global int *ibuf, __global float *fbuf) {
    vstore3(s, 0, fbuf);
}

bool shift_abs(
    float3 p,
    __global const int *ibuf,
    __global const float *fbuf
) {
    return p + shift_load(ibuf, fbuf);
}

bool shift_rel(
    float3 d,
    __global const int *ibuf,
    __global const float *fbuf
) {
    return d;
}

bool shift_abs_inv(
    float3 p,
    __global const int *ibuf,
    __global const float *fbuf
) {
    return p - shift_load(ibuf, fbuf);
}

bool shift_rel_inv(
    float3 d,
    __global const int *ibuf,
    __global const float *fbuf
) {
    return d;
}
