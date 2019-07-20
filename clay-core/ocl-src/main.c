#include "ray.h"
#include "object/sphere.h"


float2 ptos(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

__kernel void fill(
    int2 size,
    __global uchar *screen,
    float3 view,
    
    __global int *objects_int,
    __global float *objects_float,
    int size_int,
    int size_float,
    int objects_count
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    Sphere s;
    s.pos = (float3)(0.0f, 0.0f, -10.0f);
    s.rad = 2.0f;

    float2 v = ptos(pos, size);
    Ray r;
    r.start = view;
    r.dir = normalize((float3)(v, -1.0f));
    r.color = (float3)(1.0f, 1.0f, 1.0f);

    float3 mhp;
    float3 mhn;
    float md = INFINITY;
    int mi = -1;

    uchar4 color = (uchar4)(0, 0, 0, 1);
    int i = 0;
    for (i = 0; i < objects_count; ++i) {
        float3 hp;
        float3 hn;
        float d;
        Sphere s = sphere_load(objects_int + size_int*i, objects_float + size_float*i);
        if (sphere_hit(s, r, &d, &hp, &hn)) {
            if (d < md) {
                md = d;
                mhp = hp;
                mhn = hn;
                mi = i;
            }
        }
    }
    
    if (mi >= 0) {
        color = (uchar4)(convert_uchar3(255*mhn), 1);
    }

    vstore4(color, idx, screen);
}
