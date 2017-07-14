varying vec2 v_uv;
varying float v_light;

uniform sampler2D u_heli_texture;

void main() {
    vec4 t = texture2D(u_heli_texture, v_uv);
    gl_FragColor = vec4(t.xyz * v_light, t.w);
}