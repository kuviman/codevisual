#include <global>
#include <units>

varying vec2 v_v;

#ifdef VERTEX
attribute vec2 a_v;

#define RAD 150.0

void main() {
    v_v = a_v;
    gl_Position = vec4((unit_pos() + a_v * RAD) / u_map_size, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT

#define MID 0.7

void main() {
    float k;
    if (length(v_v) < MID) {
        k = 1.0;
    } else if (length(v_v) < 1.0) {
        k = (1.0 - length(v_v)) / (1.0 - MID);
    } else {
        k = 0.0;
    }
    gl_FragColor = vec4(1.0, 1.0, 1.0, k);
}
#endif