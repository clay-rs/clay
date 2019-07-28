#pragma once

#define SCENE_ARGS_DEF \
    __global const int *objects_int, \
    __global const float *objects_float, \
    int size_int, \
    int size_float, \
    int objects_count

#define SCENE_ARGS \
    objects_int, \
    objects_float, \
    size_int, \
    size_float, \
    objects_count


uchar3 scene_trace(
    Ray r,
    __global const int *objects_int,
    __global const float *objects_float,
    int size_int,
    int size_float,
    int objects_count
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
        if (hit(r, ibuf, fbuf, &d, &hp, &hn)) {
            if (d < md) {
                md = d;
                mhp = hp;
                mhn = hn;
                mi = i;
            }
        }
    }
    
    uchar3 color;
    if (mi >= 0) {
        color = convert_uchar3(255*mhn);
    } else {
        float z = 0.5f*(r.dir.z + 1.0f);
        color = convert_uchar3(255.0f*(float3)(z, z, z));
    }
    return color;
}
