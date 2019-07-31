typedef struct {
    float3 x, y, z;
} matrix3;

matrix3 matrix3_load(__global const float *fbuf) {
    matrix3 m = {
        .x = vload3(0, fbuf),
        .y = vload3(1, fbuf),
        .z = vload3(2, fbuf)
    };
    return m;
}

matrix3 matrix3_transpose(matrix3 m) {
    matrix3 t = {
        .x = (float3)(m.x.x, m.y.x, m.z.x),
        .y = (float3)(m.x.y, m.y.y, m.z.y),
        .z = (float3)(m.x.z, m.y.z, m.z.z)
    };
    return t;
}

float3 matrix3_dot(matrix3 m, float3 v) {
    return (float3)(dot(m.x, v), dot(m.y, v), dot(m.z, v));
}
