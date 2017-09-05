#include <codewars>

#ifdef VERTEX
attribute vec2 i_pos;
void main() {
    gl_Position = camera_matrix() * vec4(i_pos, 0.0, 1.0);
    gl_PointSize = 3.0;
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
#endif