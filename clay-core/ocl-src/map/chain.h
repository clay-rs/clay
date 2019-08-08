#pragma once

#include "map.h"


#define MAP_CHAIN(pref, first, second, di, df) \
    MAP_RET pref##_rel(MAP_ARGS_DEF) { \
        return second##_rel(MAP_ARGS_VB(first##_rel(MAP_ARGS), di, df)); \
    } \
    MAP_RET pref##_abs(MAP_ARGS_DEF) { \
        return second##_abs(MAP_ARGS_VB(first##_abs(MAP_ARGS), di, df)); \
    } \
    MAP_RET pref##_rel_inv(MAP_ARGS_DEF) { \
        return first##_rel_inv(MAP_ARGS_V(second##_rel_inv(MAP_ARGS_B(di, df)))); \
    } \
    MAP_RET pref##_abs_inv(MAP_ARGS_DEF) { \
        return first##_abs_inv(MAP_ARGS_V(second##_abs_inv(MAP_ARGS_B(di, df)))); \
    } \
    MAP_RET pref##_norm(MAP_ARGS_DEF) { \
        return second##_norm(MAP_ARGS_VB(first##_norm(MAP_ARGS), di, df)); \
    }
