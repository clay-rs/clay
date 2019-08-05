#include <clay_core/ray.h>
#include <__gen__/scene.h>
#include <__gen__/view.h>


__kernel void render(
    int2 size,
    __global float *color_buffer,
    __global uint *random,
    __VIEW_ARGS_DEF__,
    __SCENE_ARGS_DEF__
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;
    uint seed = random[idx];

    Ray ray = __view_emit__(&seed, pos, size, __VIEW_ARGS__);
    float3 color = __scene_trace__(&seed, ray, __SCENE_ARGS__);

    random[idx] = seed;
    vstore3(vload3(idx, color_buffer) + color, idx, color_buffer);
}
