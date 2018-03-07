varying vec2 v_vt;

uniform vec4 u_color;
uniform sampler2D u_cache_texture;
void main() {
    gl_FragColor = texture2D(u_cache_texture, v_vt) * u_color;
}