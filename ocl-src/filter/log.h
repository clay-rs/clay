#pragma once

#define LOG_FILTER_ARGS_DEF \
    float log_lower, \
    float log_upper

#define LOG_FILTER_ARGS \
    log_lower, \
    log_upper

float3 log_filter_apply(
    int2 pos, int2 size,
    __global const float *buffer,
    LOG_FILTER_ARGS_DEF
) {
    return (log(vload3(pos.x + pos.y*size.x, buffer)) - log_lower)/(log_upper - log_lower);
}
