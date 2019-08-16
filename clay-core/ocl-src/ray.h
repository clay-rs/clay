#pragma once

#define RAY_INITIAL  0
#define RAY_DIFFUSE  (1<<0)
#define RAY_TARGETED (1<<1)

typedef struct {
    float3 start;
    float3 dir;
    float3 color;
    uint history;
    int origin;
    int target;
} Ray;

Ray ray_new() {
    Ray r = {
        .start = (float3)(0.0f),
        .dir   = (float3)(0.0f),
        .color = (float3)(0.0f),
        .history = RAY_INITIAL,
        .origin = -1,
        .target = -1
    };
}
