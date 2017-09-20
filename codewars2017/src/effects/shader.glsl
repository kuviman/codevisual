#include <codewars>

#ifdef VERTEX
attribute vec3 a_v;
void main() {
    gl_Position = camera_matrix() * vec4(a_v, 1.0);
}
#endif

#ifdef FRAGMENT
uniform vec4 u_color;
void main() {
    gl_FragColor = u_color;
}
#endif