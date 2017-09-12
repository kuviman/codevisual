#include <noise2d>
#include <codewars>

varying vec3 v_pos;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 a_vn;
void main() {
    v_pos = a_v;
    gl_Position = camera_matrix() * vec4(a_v, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    float k = snoise(vec2(v_pos.z / 20.0, (v_pos.x + v_pos.y) / 300.0));
    gl_FragColor = vec4(k * 0.1 + (1.0 - k) * 0.15, 0.0, 0.0, 1.0);
}
#endif