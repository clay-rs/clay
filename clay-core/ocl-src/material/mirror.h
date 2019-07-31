#pragma once

typedef struct {
    float3 color;
} Mirror;

Mirror mirror_load(__global const int *ibuf, __global const float *fbuf) {
    Mirror s;
    //s.color = (float3)(0.5f, 0.5f, 1.0f);
    s.color = vload3(0, fbuf);
    return s;
}

void mirror_store(Mirror s, __global int *ibuf, __global float *fbuf) {
    vstore3(s.color, 0, fbuf);
}

int mirror_emit(
    Ray r,
    float3 p, float3 n,
    __global const int *ibuf,
    __global const float *fbuf,
    Ray *rr, float3 *glow
) {
    Mirror s = mirror_load(ibuf, fbuf);
    rr->start = p;
    rr->dir = r.dir - 2.0f*n*dot(n, r.dir);
    rr->color = r.color*s.color;
    return 1;
}
