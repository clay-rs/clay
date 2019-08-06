#pragma once


// Basis of the orthogonal complement to 1D subspace defined by `z`
void complement(float3 z, float3 *x, float3 *y) {
    if (fabs(z.z) < 0.5) {
        *x = (float3)(-z.y, z.x, 0.0f);
    } else {
        *x = (float3)(0.0f, -z.z, z.y);
    }
    *x = normalize(*x);
    *y = cross(z, *x);
}
