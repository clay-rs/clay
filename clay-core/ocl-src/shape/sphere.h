#pragma once

#include <shape/shape.h>

typedef struct {
    float3 pos;
    float rad;
} Sphere;

Sphere sphere_load(__global const int *ibuf, __global const float *fbuf) {
    Sphere s;
    s.pos = vload3(0, fbuf);
    s.rad = fbuf[3];
    return s;
}

void sphere_store(Sphere s, __global int *ibuf, __global float *fbuf) {
    vstore3(s.pos, 0, fbuf);
    fbuf[3] = s.rad;
}

__SHAPE_RET__ sphere_hit(
    __SHAPE_ARGS_DEF__
) {
    Sphere s = sphere_load(ibuf, fbuf);

    float l = dot(s.pos - r.start, r.dir);
    float3 c = r.start + l*r.dir;
    float3 rc = c - s.pos;
    float lr2 = dot(rc, rc);
    float rad2 = s.rad*s.rad;
    if (lr2 > rad2) {
        return false;
    }
    float dl = sqrt(rad2 - lr2);
    *d = l - dl;
    if (*d < 0.0) {
        return false;
    }
    *p = c - r.dir*dl;
    *n = (*p - s.pos)/s.rad;
    return true;
}
