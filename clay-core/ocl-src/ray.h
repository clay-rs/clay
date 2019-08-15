#pragma once

#define RAY_INITIAL 0
#define RAY_DIFFUSE 1
#define RAY_REFLECT 2
#define RAY_REFRACT 3
#define RAY_TARGET  4

typedef struct {
    float3 start;
    float3 dir;
    float3 color;
    uchar type;
    bool face;
    int origin;
    int target;
} Ray;

Ray ray_new() {
	Ray r = {
        .start = (float3)(0.0f),
        .dir   = (float3)(0.0f),
        .color = (float3)(0.0f),
		.type = RAY_INITIAL,
        .face = true,
		.origin = -1,
		.target = -1
	};
}
