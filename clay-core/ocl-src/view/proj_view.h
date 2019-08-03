#include <clay_core/ray.h>


typedef struct {
    float3 pos;
    float16 ori;
} View;

#define __VIEW_ARGS_DEF__ \
    float3 view_pos, \
    float16 view_map

#define __VIEW_ARGS__ \
    view_pos, \
    view_map


float2 ptos(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

Ray __view_emit__(
    int2 pos,
    int2 size,
    __VIEW_ARGS_DEF__
) {
    float2 v = ptos(pos, size);
    Ray ray = {
        .start = view_pos,
        .dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a),
        .color = (float3)(1.0f, 1.0f, 1.0f),
        .origin = -1,
    };
    return ray;
}
