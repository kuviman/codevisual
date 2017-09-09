#include <codewars>

varying vec2 v_vt;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec2 a_vt;
void main() {
    v_vt = a_vt;
    gl_Position = u_projection_matrix * (u_view_matrix * vec4(a_v, 0.0) + vec4(0.0, 0.0, 0.0, 1.0));
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
void main() {
    gl_FragColor = texture2D(texture, v_vt);
}
#endif