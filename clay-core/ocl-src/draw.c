__kernel void draw(
    int2 size,
    int n_passes,
    __global float *color_buffer,
    __global uchar *screen
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    uchar3 screen_color = (uchar3)(0);
    if (n_passes > 0) {
        float3 color = vload3(idx, color_buffer);
        screen_color = convert_uchar3(255.0f*clamp(color/n_passes, 0.0f, 1.0f));
    }
    vstore3(screen_color, idx, screen);
}
