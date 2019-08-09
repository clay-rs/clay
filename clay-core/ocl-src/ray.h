#pragma once

#define RAY_INITIAL 0
#define RAY_DIFFUSE 1
#define RAY_REFLECT 2
#define RAY_REFRACT 3
#define RAY_ATTRACT 4

typedef struct {
    float3 start;
    float3 dir;
    float3 color;
    uchar type;
    int origin;
    int target;
} Ray;

Ray ray_new() {
	Ray r = {
		.type = RAY_INITIAL,
		.origin = -1,
		.target = -1
	};
}
