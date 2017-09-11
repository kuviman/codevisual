#include <codewars>

varying vec2 v_vt;
varying vec4 v_color;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 a_vn;
attribute vec2 a_vt;

attribute vec2 i_pos;
attribute float i_radius;
attribute vec4 i_color;
attribute float i_height;
attribute float i_angle;

void main() {
    v_vt = a_vt;
    v_color = i_color;
    float sn = sin(i_angle);
    float cs = cos(i_angle);
    gl_Position = camera_matrix() * vec4(
        vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z) * i_radius +
        vec3(i_pos, i_height * u_sky_height), 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
void main() {
    gl_FragColor = texture2D(texture, v_vt) * v_color;
    if (gl_FragColor.w < 0.5) {
        discard;
    }
}
#endif