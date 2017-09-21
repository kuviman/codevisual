varying vec2 v_quad_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_quad_pos = a_quad_pos;
    gl_Position = vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D u_shadow_map;
void main() {
    float depth = texture2D(u_shadow_map, v_quad_pos).x;
    gl_FragColor = vec4(depth, depth, depth, 0.0);
}
#endif