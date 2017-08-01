#include <global>
#include <ground>
#include <fog>

varying vec2 v_pos;

#define WATER_LINE -1.0

#ifdef VERTEX
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    gl_Position = u_matrix * vec4(a_pos, WATER_LINE, 1.0);
}
#endif

#ifdef FRAGMENT
#define K 7.5
void main() {
    gl_FragColor = vec4(0.5, 0.7, 1.0, 0.4);
#if d_fog_enabled
    gl_FragColor.xyz *= fog_value(v_pos);
#endif
#if d_heightmap_enabled
    float h = map_height(v_pos) - WATER_LINE;
#else
    float h = -WATER_LINE;
#endif
    if (h >= 0.0) {
        discard;
    }
    if (-K < h && h < 0.0) {
        gl_FragColor.w *= -h / K;
    }
}
#endif