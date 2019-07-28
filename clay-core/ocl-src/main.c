#include "ray.h"
#include "gen/worker.h"

float2 ptos(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

__kernel void fill(
    int2 size,
    __global uchar *screen,
    float3 view_pos,
    float16 view_map,
    SCENE_ARGS_DEF
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    float2 v = ptos(pos, size);
    Ray r;
    r.start = view_pos;
    r.dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a);
    r.color = (float3)(1.0f, 1.0f, 1.0f);

    uchar3 color = scene_trace(r, SCENE_ARGS);

    vstore3(color, idx, screen);
}
