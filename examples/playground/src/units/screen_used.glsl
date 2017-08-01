#include <global>
#include <ground>
#include <units>

varying vec2 v_v;

#ifdef VERTEX
attribute vec2 a_v;
attribute float i_size;
void main() {
    v_v = a_v;
    vec2 pos = unit_pos();
#if d_heightmap_enabled
    float mapheight = map_height(pos);
#else
    float mapheight = 0.0;
#endif
#if d_is_heli
    float height = max(0.0, mapheight) + 50.0;
#else
    float height = mapheight;
#endif
    gl_Position = u_projection_matrix * (u_camera_matrix * vec4(pos, height + 0.3 * i_size, 1.0) + vec4(a_v * i_size * 2.5, 0.0, 0.0));
}
#endif

#ifdef FRAGMENT
#define MID 0.7
void main() {
    float k;
    if (length(v_v) < MID) {
        k = 1.0;
    } else if (length(v_v) < 1.0) {
        k = (1.0 - length(v_v)) / (1.0 - MID);
    } else {
        k = 0.0;
    }
    gl_FragColor = vec4(0.0, 0.0, 0.0, k);
}
#endif