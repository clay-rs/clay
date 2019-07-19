typedef struct {
    float3 pos;
    float rad;
} Sphere;

typedef struct {
    float3 start;
    float3 dir;
    float3 color;
} Ray;

bool trace(Sphere s, Ray r, float3 *p, float3 *n) {
    float3 c = r.start + dot(s.pos - r.start, r.dir)*r.dir;
    float3 rc = c - s.pos;
    float lr2 = dot(rc, rc);
    float rad2 = s.rad*s.rad;
    if (lr2 > rad2) {
        return false;
    }
    *p = c - r.dir*sqrt(rad2 - lr2);
    *n = (*p - s.pos)/s.rad;
    return true;
}

float2 screen(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

__kernel void fill(
    int width, int height,
    __global uchar *buffer
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int2 size = (int2)(width, height);
    int idx = pos.x + pos.y*width;

    Sphere s;
    s.pos = (float3)(0.0f, 0.0f, -10.0f);
    s.rad = 2.0f;

    float2 v = screen(pos, size);
    Ray r;
    r.start = (float3)(0.0f, 0.0f, 0.0f);
    r.dir = normalize((float3)(v, -1.0f));
    r.color = (float3)(1.0f, 1.0f, 1.0f);

    float3 tp;
    float3 tn;

    uchar4 color = (uchar4)(0, 0, 0, 1);
    if (trace(s, r, &tp, &tn)) {
        color = (uchar4)(convert_uchar3(255*tn), 1);
    }

    vstore4(color, idx, buffer);
}
