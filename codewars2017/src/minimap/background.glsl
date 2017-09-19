varying vec2 v_quad_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
uniform mat4 minimatrix;
void main() {
    v_quad_pos = a_quad_pos;
    gl_Position = minimatrix * vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D map;
void main() {
    gl_FragColor = vec4(texture2D(map, v_quad_pos).xyz, 0.5);
}
#endif