#pragma once

#include <clay_core/map/map.h>
#include <clay_core/object/object.h>


#define MAP_OBJECT_FN_DEF(map_object, object, map, sdi, sdf) \
    MAP_SHAPE_FN_DEF(map_object, object, map, sdi, sdf) \
    OBJECT_BOUNCE_RET map_object##_bounce(OBJECT_BOUNCE_ARGS_DEF) { \
        return object##_bounce(OBJECT_BOUNCE_ARGS); \
    }
