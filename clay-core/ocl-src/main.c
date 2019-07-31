#include "ray.h"
#include "__gen__/scene.h"
#include "__gen__/view.h"


__kernel void fill(
    int2 size,
    __global uchar *screen,
    __VIEW_ARGS_DEF__,
    __SCENE_ARGS_DEF__
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    Ray r = __view_emit__(pos, size, __VIEW_ARGS__);
    float3 color = __scene_trace__(r, __SCENE_ARGS__);

    uchar3 cc = convert_uchar3(255.0f*clamp(color, 0.0f, 1.0f));

    vstore3(cc, idx, screen);
}
