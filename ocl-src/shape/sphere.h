#pragma once

#include <clay_core/ray.h>
#include <clay_core/random.h>
#include <clay_core/matrix.h>
#include <clay_core/linalg.h>
#include <clay_core/shape/shape.h>
#include <clay_core/shape/target.h>


SHAPE_HIT_RET unit_sphere_hit(
    SHAPE_HIT_ARGS_DEF
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

TARGET_SAMPLE_RET sphere_target_sample(
    TARGET_SAMPLE_ARGS_DEF
) {
    float rad = fbuf[0];
    float3 spos = vload3(0, fbuf + 1);

    float3 sdir = spos - pos;
    float len2 = dot(sdir, sdir);

    float sin_alpha_2 = (rad*rad)/len2;
    if (sin_alpha_2 >= 1.0f) {
        *dir = random_sphere(seed);
        return 2.0f;
    }
    float cos_alpha = sqrt(1.0f - sin_alpha_2);

    sdir /= sqrt(len2);
    float3 rand_dir = random_sphere_cap(seed, cos_alpha);
    matrix3 basis = { .z = sdir };
    complement(basis.z, &basis.x, &basis.y);
    *dir = matrix3_dot(matrix3_transpose(basis), rand_dir);

    return 1.0f - cos_alpha;
}
