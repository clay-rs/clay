#pragma once

#include <shape/shape.h>

typedef struct {
    float3 pos;
    float rad;
} Sphere;

Sphere sphere_load(__global const int *ibuf, __global const float *fbuf) {
    Sphere s;
    s.pos = (float3)(0.0f);
    s.rad = 1.0f;
    return s;
}

__SHAPE_RET__ sphere_hit(
    __SHAPE_ARGS_DEF__
) {
    Sphere s = sphere_load(ibuf, fbuf);

    float l = dot(s.pos - ray.start, ray.dir);
    float3 c = ray.start + l*ray.dir;
    float3 rc = c - s.pos;
    float lr2 = dot(rc, rc);
    float rad2 = s.rad*s.rad;
    if (lr2 > rad2) {
        return false;
    }
    float dl = sqrt(rad2 - lr2);
    *dist = l - dl;
    if (*dist < 0.0) {
        return false;
    }
    *pos = c - ray.dir*dl;
    *norm = (*pos - s.pos)/s.rad;
    return true;
}
