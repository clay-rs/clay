#pragma once


uint random_next(uint *seed)
{
    return (*seed = 1103515245**seed + 12345);
}

float random_unif(uint *seed)
{
    return (float)random_next(seed)/(float)0xffffffff;
}

void get_2d_perp_basis(float3 z, float3 *x, float3 *y) {
    float3 nx, ny;
    if(dot((float3)(0.0f,0.0f,1.0f),z) < 0.6 && dot((float3)(0.0f,0.0f,1.0f),z) > -0.6)
    {
        nx = (float3)(0.0f,0.0f,1.0f);
    }
    else
    {
        nx = (float3)(1.0f,0.0f,0.0f);
    }
    ny = normalize(cross(nx,z));
    nx = cross(ny,z);
    *x = nx;
    *y = ny;
}

float3 random_hemisphere(float3 norm, uint *seed) {
    float3 nx, ny;
    get_2d_perp_basis(norm, &nx, &ny);
    float phi = 2.0f*M_PI_F*random_unif(seed);
    float theta = acos(1.0f - 2.0f*random_unif(seed));
    return nx*cos(phi)*sin(theta) + ny*sin(phi)*sin(theta) + norm*cos(theta);
}

float3 random_hemisphere_cos(float3 norm, uint *seed) {
    float3 nx, ny;
    get_2d_perp_basis(norm, &nx, &ny);
    float phi = 2.0f*M_PI_F*random_unif(seed);
    float theta = acos(1.0f - 2.0f*random_unif(seed))/2.0f;
    return nx*cos(phi)*sin(theta) + ny*sin(phi)*sin(theta) + norm*cos(theta);
}

float3 random_sphere_cap(float3 norm, float cos_alpha, uint *seed) {
    float3 nx, ny;
    get_2d_perp_basis(norm, &nx, &ny);
    float phi = 2.0f*M_PI_F*random_unif(seed);
    float theta = acos(1.0f - (1.0 - cos_alpha)*random_unif(seed));
    return nx*cos(phi)*sin(theta) + ny*sin(phi)*sin(theta) + norm*cos(theta);
}

float2 random_disk(uint *seed)
{
    float r = sqrt(random_unif(seed));
    float phi = 2.0*M_PI_F*random_unif(seed);
    return r*(float2)(cos(phi),sin(phi));   
}
