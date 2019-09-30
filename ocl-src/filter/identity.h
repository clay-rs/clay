#pragma once

#include <clay_core/filter/filter.h>

// placeholder args
#define IDENTITY_FILTER_ARGS_DEF int _placeholder
#define IDENTITY_FILTER_ARGS 0


FILTER_RET identity_filter_apply(
    FILTER_ARGS_DEF,
    IDENTITY_FILTER_ARGS_DEF
) {
    int idx = pos.x + pos.y*size.x;
    return vload3(idx, buffer);
}
