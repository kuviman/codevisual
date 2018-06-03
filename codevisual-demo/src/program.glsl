varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_v;
attribute vec2 a_vt;
void main() {
    v_vt = a_vt;
    gl_Position = vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_texture;
void main() {
    gl_FragColor = texture2D(u_texture, v_vt) * u_color;
}
#endif