#pragma once

#include <clay_core/map/map.h>


MAP_RET scale_rel(MAP_ARGS_DEF) {
    return v*fbuf[0];
}

MAP_RET scale_abs(MAP_ARGS_DEF) {
    return scale_rel(MAP_ARGS);
}

MAP_RET scale_rel_inv(MAP_ARGS_DEF) {
    return v/fbuf[0];
}

MAP_RET scale_abs_inv(MAP_ARGS_DEF) {
    return scale_rel_inv(MAP_ARGS);
}

MAP_RET scale_norm(MAP_ARGS_DEF) {
    return v;
}
