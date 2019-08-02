#pragma once

#include "shape.h"


__SHAPE_RET__ sphere_hit(
    __SHAPE_ARGS_DEF__
) {
    // t^2 - 2*b*t + c = 0
    float b = -dot(ray.dir, ray.start);
    float c = dot(ray.start, ray.start) - 1.0f;
    float d = b*b - c;
    if (d < 0.0f) {
        return false;
    }
    d = sqrt(d);
    float e = b - d;
    if (e < 0.0f) {
        return false;
    }
    *enter = e;
    *exit = b + d;
    *norm = ray.start + ray.dir*e;
    return true;
}
