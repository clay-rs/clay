__kernel void fill(
    int width, int height,
    __global uchar *buffer
) {
    int idx = get_global_id(0);

    int2 pos = (int2)(idx % width, idx / width);
    int2 size = (int2)(width, height);

    uchar4 color = (uchar4)(convert_uchar2((255*pos)/size), 0, 1);

    vstore4(color, idx, buffer);
}
