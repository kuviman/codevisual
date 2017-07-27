#include <global>
#include <fog>

varying vec2 v_pos;

#ifdef VERTEX
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
    gl_Position = u_matrix * vec4(a_pos, -1.0, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = vec4(0.5, 0.7, 1.0, 0.4);
    gl_FragColor.xyz *= fog_value(v_pos);
}
#endif