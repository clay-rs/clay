#pragma once

#include <clay_core/shape/shape.h>
#include <clay_core/material/material.h>

#define OBJECT_HIT_RET               SHAPE_HIT_RET
#define OBJECT_HIT_RET_BAD           SHAPE_HIT_RET_BAD
#define OBJECT_HIT_ARGS_DEF          SHAPE_HIT_ARGS_DEF
#define OBJECT_HIT_ARGS              SHAPE_HIT_ARGS
#define OBJECT_HIT_ARGS_B(di, df)    SHAPE_HIT_ARGS_B(di, df)
#define OBJECT_HIT_ARGS_R(r)         SHAPE_HIT_ARGS_R(r)

#define OBJECT_BOUNCE_RET            MATERIAL_BOUNCE_RET
#define OBJECT_BOUNCE_RET_BAD        MATERIAL_BOUNCE_RET_BAD
#define OBJECT_BOUNCE_ARGS_DEF       MATERIAL_BOUNCE_ARGS_DEF
#define OBJECT_BOUNCE_ARGS           MATERIAL_BOUNCE_ARGS
#define OBJECT_BOUNCE_ARGS_B(di, df) MATERIAL_BOUNCE_ARGS_B(di, df)
