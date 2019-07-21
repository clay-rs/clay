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
    
    __global const int *objects_int,
    __global const float *objects_float,
    int size_int,
    int size_float,
    int objects_count
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    float2 v = ptos(pos, size);
    Ray r;
    r.start = view_pos;
    r.dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a);
    r.color = (float3)(1.0f, 1.0f, 1.0f);

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

    vstore3(color, idx, screen);
}
