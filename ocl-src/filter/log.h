#pragma once

#include <clay_core/filter/filter.h>

#define LOG_FILTER_ARGS_DEF \
    float log_lower, \
    float log_upper

#define LOG_FILTER_ARGS \
    log_lower, \
    log_upper

FILTER_RET log_filter_apply(
    FILTER_ARGS_DEF,
    LOG_FILTER_ARGS_DEF
) {
    return (log(vload3(pos.x + pos.y*size.x, buffer)) - log_lower)/(log_upper - log_lower);
}
