#include <global>

#ifdef VERTEX
attribute vec3 a_v;
void main() {
    gl_Position = u_matrix() * vec4(a_v, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
#endif