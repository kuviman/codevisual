#include <codewars>

varying float v_start_time;
varying vec4 v_color;

#ifdef VERTEX
attribute vec4 a_color;
attribute float a_start_time;
attribute vec3 a_v;
attribute float a_size;
void main() {
    v_color = a_color;
    v_start_time = a_start_time;
    gl_Position = u_projection_matrix * (u_view_matrix * vec4(a_v, 1.0) + vec4(0.0, 0.0, 3.0, 0.0));
    gl_PointSize = a_size * u_framebuffer_size.y / gl_Position.w;
}
#endif

#ifdef FRAGMENT
uniform sampler2D u_texture;
void main() {
    gl_FragColor = texture2D(u_texture, gl_PointCoord) * v_color;
    gl_FragColor.w *= pow(1.0 - (u_current_time - v_start_time) / (5.0 / 60.0), 0.5);
}
#endif