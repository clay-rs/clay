#pragma once

#define GLARE_FILTER_ARGS_DEF \
    float glare_strength

#define GLARE_FILTER_ARGS \
    glare_strength

float3 glare_filter_apply(
    int2 pos, int2 size,
    __global const float *buffer,
    GLARE_FILTER_ARGS_DEF
) {
    int i;

    float3 addition = (float3)(0.0f);
    for (i = 0; i < pos.x; ++i) {
        addition += vload3(i + pos.y*size.x, buffer)/(pos.x - i);
    }
    for (i = pos.x + 1; i < size.x; ++i) {
        addition += vload3(i + pos.y*size.x, buffer)/(i - pos.x);
    }

    for (i = 0; i < pos.y; ++i) {
        addition += vload3(pos.x + i*size.x, buffer)/(pos.y - i);
    }
    for (i = pos.y + 1; i < size.y; ++i) {
        addition += vload3(pos.x + i*size.x, buffer)/(i - pos.y);
    }

    addition *= glare_strength;

    return vload3(pos.x + pos.y*size.x, buffer) + addition;
}
