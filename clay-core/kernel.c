


typedef struct {
    float3 start;
    float3 dir;
    float3 color;
    int origin;
} Ray;







#define __SHAPE_RET__ bool

#define __SHAPE_ARGS_DEF__ \
    Ray ray, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *enter, float *exit, float3 *norm

#define __SHAPE_ARGS__ \
    ray, ibuf, fbuf, enter, exit, norm

#define __SHAPE_ARGS_B__(di, df) \
    ray, ibuf + (di), fbuf + (df), enter, exit, norm

#define __SHAPE_ARGS_R__(r) \
    (r), ibuf, fbuf, enter, exit, norm


float _hit_norm(float3 near, float3 *norm) {
    bool xy = near.x > near.y;
    bool yz = near.y > near.z;
    bool xz = near.x > near.z;
    float dist = 0.0;
    if (xy && xz) {
        dist = near.x;
        norm->x = 1.0f;
    } else if (yz) {
        dist = near.y;
        norm->y = 1.0f;
    } else {
        dist = near.z;
        norm->z = 1.0f;
    }
    return dist;
}


__SHAPE_RET__ cube_hit(
    __SHAPE_ARGS_DEF__
) {
    const float3 cmax = (float3)(1.0f);
    const float3 cmin = (float3)(-1.0f);

    float3 inv_dir = 1.0f/ray.dir;

    float3 vmin = (cmin - ray.start)*inv_dir;
    float3 vmax = (cmax - ray.start)*inv_dir;

    float3 near = min(vmin, vmax);
    float3 far = max(vmin, vmax);

    float3 norm_in = (float3)(0.0f);
    float dist_in = _hit_norm(near, &norm_in);
    norm_in *= sign(ray.dir);

    float3 norm_out = (float3)(0.0f);
    float dist_out = -_hit_norm(-far, &norm_out);
    norm_out *= -sign(ray.dir);

    if (dist_in < 0.0f || dist_in > dist_out) {
        return false;
    }

    *enter = dist_in;
    *exit = dist_out;
    *norm = norm_in;
    return true;
}




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





#define __MAP_RET__ float3

#define __MAP_ARGS_DEF__ \
    float3 v, \
    __global const int *ibuf, \
    __global const float *fbuf

#define __MAP_ARGS__ \
    v, ibuf, fbuf

#define __MAP_ARGS_VB__(v, di, df) \
    (v), ibuf + (di), fbuf + (df)

#define __MAP_ARGS_V__(v) \
    __MAP_ARGS_VB__(v, 0, 0)

#define __MAP_ARGS_B__(di, df) \
    __MAP_ARGS_VB__(v, di, df)

#define MAP_SHAPE_FN_DEF(map_shape_fn, shape_fn, map_pref, mdi, mdf) \
    __SHAPE_RET__ map_shape_fn(__SHAPE_ARGS_DEF__) { \
        Ray new_ray = ray; \
        new_ray.start = map_pref##_abs_inv(__MAP_ARGS_VB__(ray.start, mdi, mdf)); \
        float3 new_dir = map_pref##_rel_inv(__MAP_ARGS_VB__(ray.dir, mdi, mdf)); \
        float lenf = 1.0f/length(new_dir); \
        new_ray.dir = new_dir*lenf; \
        __SHAPE_RET__ ret = shape_fn(__SHAPE_ARGS_R__(new_ray)); \
        if (ret) { \
            *enter *= lenf; \
            *exit *= lenf; \
            *norm = normalize(map_pref##_norm(__MAP_ARGS_VB__(*norm, mdi, mdf))); \
        } \
        return ret; \
    }


__MAP_RET__ affine_rel(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3);
    return matrix3_dot(linear, v);
}

__MAP_RET__ affine_abs(__MAP_ARGS_DEF__) {
    float3 shift = vload3(0, fbuf);
    return affine_rel(__MAP_ARGS__) + shift;
}

__MAP_RET__ affine_rel_inv(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(linear, v);
}

__MAP_RET__ affine_abs_inv(__MAP_ARGS_DEF__) {
    float3 shift = vload3(0, fbuf);
    return affine_rel_inv(__MAP_ARGS_V__(v - shift));
}

__MAP_RET__ affine_norm(__MAP_ARGS_DEF__) {
    matrix3 linear = matrix3_load(fbuf + 3 + 9);
    return matrix3_dot(matrix3_transpose(linear), v);
}
MAP_SHAPE_FN_DEF(__cube_hit_affine_4852da00a9acb7de__, cube_hit, affine, 0, 0)






#define __MATERIAL_RET__ int

#define __MATERIAL_ARGS_DEF__ \
    Ray r, \
    float3 p, float3 n, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    Ray *rr, float3 *glow

#define __MATERIAL_ARGS__ \
    r, p, n, ibuf, fbuf, rr, glow

#define __MATERIAL_ARGS_B__(di, df) \
    r, p, n, ibuf + (di), fbuf + (df), rr, glow

typedef struct {
    float3 color;
} Mirror;

Mirror mirror_load(__global const int *ibuf, __global const float *fbuf) {
    Mirror s;
    //s.color = (float3)(0.5f, 0.5f, 1.0f);
    s.color = vload3(0, fbuf);
    return s;
}

__MATERIAL_RET__ mirror_emit(
    __MATERIAL_ARGS_DEF__
) {
    Mirror s = mirror_load(ibuf, fbuf);
    rr->start = p;
    rr->dir = r.dir - 2.0f*n*dot(n, r.dir);
    rr->color = r.color*s.color;
    return 1;
}
__MATERIAL_RET__ __mirror_emit_a835c9f037f2f76f__(

__MATERIAL_ARGS_DEF__
) {
	return mirror_emit(__MATERIAL_ARGS_B__(0, 21));
}
#define __object_hit__ __cube_hit_affine_4852da00a9acb7de__
#define __object_emit__ __mirror_emit_a835c9f037f2f76f__



#define __SCENE_ARGS_DEF__ \
    __global const int *objects_int, \
    __global const float *objects_float, \
    int size_int, \
    int size_float, \
    int objects_count

#define __SCENE_ARGS__ \
    objects_int, \
    objects_float, \
    size_int, \
    size_float, \
    objects_count


float3 scene_trace(
    Ray ray,
    int depth,
    __SCENE_ARGS_DEF__
) {
    int hit_idx = -1;
    float hit_enter = INFINITY;
    float hit_exit = 0.0f;
    float3 hit_norm;

    int i = 0;
    for (i = 0; i < objects_count; ++i) {
        float enter, exit;
        float3 norm;

        if (ray.origin == i) {
            continue;
        }

        __global const int *ibuf = objects_int + size_int*i;
        __global const float *fbuf = objects_float + size_float*i;
        if (__object_hit__(ray, ibuf, fbuf, &enter, &exit, &norm)) {
            if (enter < hit_enter) {
                hit_enter = enter;
                hit_exit = exit;
                hit_norm = norm;
                hit_idx = i;
            }
        }
    }

    float3 color = (float3)(0.0f);
    if (hit_idx >= 0 && depth < 4) {
        Ray new_ray;
        float3 glow = (float3)(0.0f);
        float3 hit_pos = ray.start + ray.dir*hit_enter;

        __global const int *ibuf = objects_int + size_int*hit_idx;
        __global const float *fbuf = objects_float + size_float*hit_idx;
        int num_rays = __object_emit__(ray, hit_pos, hit_norm, ibuf, fbuf, &new_ray, &glow);
        new_ray.origin = hit_idx;
        color += glow;
        if (num_rays > 0) {
            color += scene_trace(new_ray, depth + 1, __SCENE_ARGS__);
        }
    } else {
        float z = 0.5f*(ray.dir.z + 1.0f);
        color = ray.color*(float3)(z, z, z);
    }
    return color;
}

float3 __scene_trace__(
    Ray ray,
    __SCENE_ARGS_DEF__
) {
    return scene_trace(ray, 0, __SCENE_ARGS__);
}





typedef struct {
    float3 pos;
    float16 ori;
} View;

#define __VIEW_ARGS_DEF__ \
    float3 view_pos, \
    float16 view_map

#define __VIEW_ARGS__ \
    view_pos, \
    view_map


float2 ptos(int2 pos, int2 size) {
    float2 p = convert_float2(pos) - 0.5f*convert_float2(size);
    p.y = -p.y;
    return p/(float)size.y;
}

Ray __view_emit__(
    int2 pos,
    int2 size,
    __VIEW_ARGS_DEF__
) {
    float2 v = ptos(pos, size);
    Ray ray = {
        .start = view_pos,
        .dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a),
        .color = (float3)(1.0f, 1.0f, 1.0f),
        .origin = -1,
    };
    return ray;
}


__kernel void fill(
    int2 size,
    __global uchar *screen,
    __VIEW_ARGS_DEF__,
    __SCENE_ARGS_DEF__
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    Ray ray = __view_emit__(pos, size, __VIEW_ARGS__);
    float3 color = __scene_trace__(ray, __SCENE_ARGS__);

    uchar3 cc = convert_uchar3(255.0f*clamp(color, 0.0f, 1.0f));

    vstore3(cc, idx, screen);
}
