#pragma once


// Random 32-bit integer from linear congruential generator
uint _random(uint *seed) {
    return (*seed = 1103515245**seed + 12345);
}

// Uniform random distribution between 0 (including) and 1 (excluding)
float random_uniform(uint *seed) {
    return (float)_random(seed)/(float)0x100000000;
}

// Uniform distribution on the surface of the unit sphere
float3 random_sphere(uint *seed) {
    float phi = 2.0f*M_PI_F*random_uniform(seed);
    float cos_theta = 1.0f - 2.0f*random_uniform(seed);
    float sin_theta = sqrt(1.0f - cos_theta*cos_theta);
    return (float3)(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

// Uniform distribution on the surface of the z > 0 half of the unit sphere
float3 random_hemisphere(uint *seed) {
    float phi = 2.0f*M_PI_F*random_uniform(seed);
    float cos_theta = random_uniform(seed);
    float sin_theta = sqrt(1.0f - cos_theta*cos_theta);
    return (float3)(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

float3 random_hemisphere_cosine(uint *seed) {
    float phi = 2.0f*M_PI_F*random_uniform(seed);
    float sqr_cos_theta = random_uniform(seed);
    float cos_theta = sqrt(sqr_cos_theta);
    float sin_theta = sqrt(1.0f - sqr_cos_theta);
    return (float3)(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}

float3 random_sphere_cap(uint *seed, float cos_alpha) {
    float phi = 2.0f*M_PI_F*random_uniform(seed);
    float cos_theta = 1.0f - (1.0f - cos_alpha)*random_uniform(seed);
    float sin_theta = sqrt(1.0f - cos_theta*cos_theta);
    return (float3)(cos(phi)*sin_theta, sin(phi)*sin_theta, cos_theta);
}
/*
float2 random_disk(uint *seed)
{
    float r = sqrt(random_uniform(seed));
    float phi = 2.0*M_PI_F*random_uniform(seed);
    return r*(float2)(cos(phi),sin(phi));   
}
*/
