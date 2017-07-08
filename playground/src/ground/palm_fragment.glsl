varying vec2 v_vt;

uniform sampler2D u_palm_texture;

void main() {
    gl_FragColor = texture2D(u_palm_texture, vec2(v_vt.x, -v_vt.y));
    if (gl_FragColor.w < 0.5) {
        discard;
    }
    gl_FragColor.w /= 2.0;
}