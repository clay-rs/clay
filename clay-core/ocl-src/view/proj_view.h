#include <clay_core/ray.h>
#include <clay_core/random.h>


typedef struct {
    float3 pos;
    float16 ori;
} View;

#define VIEW_ARGS_DEF \
    float3 view_pos, \
    float16 view_map

#define VIEW_ARGS \
    view_pos, \
    view_map


float2 ptos(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

float2 ptos_rand(uint *seed, int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    p += (float2)(random_uniform(seed), random_uniform(seed)) - 0.5f;
    return p/(float)size.y;
}


Ray __view_emit(
    uint *seed,
    int2 pos,
    int2 size,
    VIEW_ARGS_DEF
) {
    float2 v = ptos_rand(seed, pos, size);
    Ray ray = {
        .start = view_pos,
        .dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a),
        .color = (float3)(1.0f, 1.0f, 1.0f),
        .type = RAY_INITIAL,
        .origin = -1,
        .target = -1,
    };
    return ray;
}
