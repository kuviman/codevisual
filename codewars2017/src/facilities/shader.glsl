#include <codewars>
#include <shadow>

varying vec3 v_pos;
varying vec2 v_vt;
varying float v_light;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 a_vn;
attribute vec2 a_vt;

attribute vec2 i_pos;

#define RADIUS float(32.0)

void main() {
    v_vt = a_vt;
    v_light = get_light(a_vn);
    v_pos = a_v * RADIUS + vec3(i_pos, 0.0);
#ifdef SHADOW_CAST_MATERIAL
    set_shadow_pos(v_pos);
#else
    gl_Position = camera_matrix() * vec4(v_pos, 1.0);
#endif
}
#endif

#ifdef FRAGMENT
uniform sampler2D u_texture;
void main() {
    gl_FragColor = texture2D(u_texture, v_vt);
    gl_FragColor.xyz *= min(v_light, get_shadow(v_pos)) * (1.0 - u_ambient_light) + u_ambient_light;
    gl_FragColor.w = 0.5;
}
#endif