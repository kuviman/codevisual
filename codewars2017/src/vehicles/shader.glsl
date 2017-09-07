#include <codewars>

varying vec2 v_v;
varying vec4 v_color;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 i_pos;
attribute float i_radius;
attribute vec4 i_color;
attribute float i_height;

void main() {
    v_v = a_v;
    v_color = i_color;
    gl_Position = u_projection_matrix * (u_view_matrix * vec4(i_pos, i_height * u_sky_height, 1.0) + vec4(a_v * i_radius, 0.0, 0.0));
}
#endif

#ifdef FRAGMENT
void main() {
    float ln = length(v_v);
    if (ln > 1.0) {
        discard;
    }
    gl_FragColor = v_color;
}
#endif