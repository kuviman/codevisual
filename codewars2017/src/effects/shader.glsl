#include <codewars>

varying vec4 v_color;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec4 a_color;
void main() {
    v_color = a_color;
    gl_Position = camera_matrix() * vec4(a_v, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = v_color;
}
#endif