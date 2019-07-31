


typedef struct {
    float3 start;
    float3 dir;
    float3 color;
} Ray;







#define __SHAPE_RET__ bool

#define __SHAPE_ARGS_DEF__ \
    Ray ray, \
    __global const int *ibuf, \
    __global const float *fbuf, \
    float *dist, float3 *pos, float3 *norm

#define __SHAPE_ARGS__ \
    ray, ibuf, fbuf, dist, pos, norm

#define __SHAPE_ARGS_B__(di, df) \
    ray, ibuf + (di), fbuf + (df), dist, pos, norm

#define __SHAPE_ARGS_R__(r) \
    (r), ibuf, fbuf, dist, pos, norm

typedef struct {
    float3 pos;
    float rad;
} Sphere;

Sphere sphere_load(__global const int *ibuf, __global const float *fbuf) {
    Sphere s;
    s.pos = (float3)(0.0f);
    s.rad = 1.0f;
    return s;
}

__SHAPE_RET__ sphere_hit(
    __SHAPE_ARGS_DEF__
) {
    Sphere s = sphere_load(ibuf, fbuf);

    float l = dot(s.pos - ray.start, ray.dir);
    float3 c = ray.start + l*ray.dir;
    float3 rc = c - s.pos;
    float lr2 = dot(rc, rc);
    float rad2 = s.rad*s.rad;
    if (lr2 > rad2) {
        return false;
    }
    float dl = sqrt(rad2 - lr2);
    *dist = l - dl;
    if (*dist < 0.0) {
        return false;
    }
    *pos = c - ray.dir*dl;
    *norm = (*pos - s.pos)/s.rad;
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
        new_ray.dir = normalize(map_pref##_rel_inv(__MAP_ARGS_VB__(ray.dir, mdi, mdf))); \
        __SHAPE_RET__ ret = shape_fn(__SHAPE_ARGS_R__(new_ray)); \
        if (ret) { \
            *pos = map_pref##_abs(__MAP_ARGS_VB__(*pos, mdi, mdf)); \
            *norm = normalize(map_pref##_norm(__MAP_ARGS_VB__(*norm, mdi, mdf))); \
            *dist = dot(*pos - ray.start, ray.dir); \
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
MAP_SHAPE_FN_DEF(__sphere_hit_affine_bcf42a589a78394e__, sphere_hit, affine, 0, 0)






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
__MATERIAL_RET__ __mirror_emit_ee7fc7421b23d6c9__(

__MATERIAL_ARGS_DEF__
) {
	return mirror_emit(__MATERIAL_ARGS_B__(0, 21));
}
#define __object_hit__ __sphere_hit_affine_bcf42a589a78394e__
#define __object_emit__ __mirror_emit_ee7fc7421b23d6c9__



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
    Ray r,
    int depth,
    __SCENE_ARGS_DEF__
) {
    float3 mhp;
    float3 mhn;
    float md = INFINITY;
    int mi = -1;

    int i = 0;
    for (i = 0; i < objects_count; ++i) {
        float3 hp;
        float3 hn;
        float d;

        __global const int *ibuf = objects_int + size_int*i;
        __global const float *fbuf = objects_float + size_float*i;
        if (__object_hit__(r, ibuf, fbuf, &d, &hp, &hn)) {
            if (d < md) {
                md = d;
                mhp = hp;
                mhn = hn;
                mi = i;
            }
        }
    }

    float3 color = (float3)(0.0f);
    if (mi >= 0 && depth < 4) {
        Ray rr;
        float3 glow = (float3)(0.0f);
        __global const int *ibuf = objects_int + size_int*mi;
        __global const float *fbuf = objects_float + size_float*mi;
        int nr = __object_emit__(r, mhp, mhn, ibuf, fbuf, &rr, &glow);
        if (nr > 0) {
            color = scene_trace(rr, depth + 1, __SCENE_ARGS__);
        }
        color += glow;
    } else {
        float z = 0.5f*(r.dir.z + 1.0f);
        color = r.color*(float3)(z, z, z);
    }
    return color;
}

float3 __scene_trace__(
    Ray r,
    __SCENE_ARGS_DEF__
) {
    return scene_trace(r, 0, __SCENE_ARGS__);
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
    Ray r;
    r.start = view_pos;
    r.dir = normalize(v.x*view_map.s012 + v.y*view_map.s456 - 1.0f*view_map.s89a);
    r.color = (float3)(1.0f, 1.0f, 1.0f);
    return r;
}


__kernel void fill(
    int2 size,
    __global uchar *screen,
    __VIEW_ARGS_DEF__,
    __SCENE_ARGS_DEF__
) {
    int2 pos = (int2)(get_global_id(0), get_global_id(1));
    int idx = pos.x + pos.y*size.x;

    Ray r = __view_emit__(pos, size, __VIEW_ARGS__);
    float3 color = __scene_trace__(r, __SCENE_ARGS__);

    uchar3 cc = convert_uchar3(255.0f*clamp(color, 0.0f, 1.0f));

    vstore3(cc, idx, screen);
}
