varying vec2 v_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_pos = a_quad_pos;
    gl_Position = vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform float alpha;
uniform sampler2D texture;
void main() {
    gl_FragColor = texture2D(texture, v_pos);
    gl_FragColor.w *= alpha;
}
#endif