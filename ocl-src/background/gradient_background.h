#pragma once

#define BACKGROUND_ARGS_DEF \
    float3 bg_top, float3 bg_bottom

#define BACKGROUND_ARGS \
    bg_top, bg_bottom

float3 __background(
    Ray ray,
    BACKGROUND_ARGS_DEF
) {
    float z = 0.5f*(ray.dir.z + 1.0f);
    return ray.color*(z*bg_top + (1.0f - z)*bg_bottom);
}
