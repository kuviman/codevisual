#include <noise2d>
#include <codewars>

varying vec3 pos;

#ifdef VERTEX
attribute vec3 a_v;
void main() {
    pos = a_v;
    gl_Position = camera_matrix() * vec4(a_v.xy, a_v.z * u_sky_height, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    float c = max(0.0, snoise(vec2(pos.x, pos.z * 10.0) + vec2(0.0, 20.0) * u_current_time) * 2.0 - 1.0);
    if (pos.z > 0.5) {
        c *= (1.0 - pos.z) / 0.5;
    }
    gl_FragColor = vec4(0.0, 0.0, 1.0, 0.3 * c);
}
#endif