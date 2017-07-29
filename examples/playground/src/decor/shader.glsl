#include <global>
#include <ground>
#include <fog>

varying vec2 v_vt;
varying vec3 v_pos;

#ifdef VERTEX
attribute vec3 a_pos;
attribute vec2 a_vt;

attribute vec2 i_pos;
attribute float i_size;

void main() {
    v_vt = a_vt;
    v_pos = vec3(i_pos, map_height(i_pos)) + a_pos * i_size;
    gl_Position = u_matrix * vec4(v_pos, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D u_texture;
uniform sampler2D u_screen_used_texture;
uniform vec2 FRAMEBUFFER_SIZE;

void main() {
    gl_FragColor = texture2D(u_texture, vec2(v_vt.x, -v_vt.y));
    if (gl_FragColor.w < 0.5) {
        discard;
    }
    gl_FragColor.xyz *= fog_value(v_pos.xy);
#if PALM
    #define MIN_VIS 0.4
    #define MAX_VIS 1.0
    gl_FragColor.w = texture2D(u_screen_used_texture, gl_FragCoord.xy / FRAMEBUFFER_SIZE).x * (MAX_VIS - MIN_VIS) + MIN_VIS;
#else
    gl_FragColor.w = 1.0;
#endif
}
#endif