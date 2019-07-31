#pragma once

#define __SCENE_ARGS_DEF__ \
    __global const int *objects_int, \
    __global const float *objects_float, \
    int size_int, \
    int size_float, \
    int objects_count

#define __SCENE_ARGS__ \
    objects_int, \
    objects_float, \
    size_int, \
    size_float, \
    objects_count


float3 scene_trace(
    Ray r,
    int depth,
    __SCENE_ARGS_DEF__
) {
    float3 mhp;
    float3 mhn;
    float md = INFINITY;
    int mi = -1;

    int i = 0;
    for (i = 0; i < objects_count; ++i) {
        float3 hp;
        float3 hn;
        float d;
        
        __global const int *ibuf = objects_int + size_int*i;
        __global const float *fbuf = objects_float + size_float*i;
        if (__object_hit__(r, ibuf, fbuf, &d, &hp, &hn)) {
            if (d < md) {
                md = d;
                mhp = hp;
                mhn = hn;
                mi = i;
            }
        }
    }
    
    float3 color = (float3)(0.0f);
    if (mi >= 0 && depth < 4) {
        Ray rr;
        float3 glow = (float3)(0.0f);
        __global const int *ibuf = objects_int + size_int*mi;
        __global const float *fbuf = objects_float + size_float*mi;
        int nr = __object_emit__(r, mhp, mhn, ibuf, fbuf, &rr, &glow);
        if (nr > 0) {
            color = scene_trace(rr, depth + 1, __SCENE_ARGS__);
        }
        color += glow;
    } else {
        float z = 0.5f*(r.dir.z + 1.0f);
        color = r.color*(float3)(z, z, z);
    }
    return color;
}

float3 __scene_trace__(
    Ray r,
    __SCENE_ARGS_DEF__
) {
    return scene_trace(r, 0, __SCENE_ARGS__);
}
