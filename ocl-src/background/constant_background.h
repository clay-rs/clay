#pragma once

#define BACKGROUND_ARGS_DEF \
    float3 bg_color

#define BACKGROUND_ARGS \
    bg_color

float3 __background(
    Ray ray,
    BACKGROUND_ARGS_DEF
) {
    return ray.color*bg_color;
}
