#include <clay_core/ray.h>
#include <__gen/scene.h>
#include <__gen/view.h>


__kernel void render(
    int2 size,
    __global float *color_buffer,
    __global uint *random,
    VIEW_ARGS_DEF,
    SCENE_ARGS_DEF
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;
    uint seed = random[idx];

    Ray ray = __view_emit(&seed, pos, size, VIEW_ARGS);
    float3 color = __scene_trace(&seed, ray, SCENE_ARGS);

    random[idx] = seed;
    vstore3(vload3(idx, color_buffer) + color, idx, color_buffer);
}
