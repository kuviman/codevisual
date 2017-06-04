precision mediump float;

varying vec2 v_uv;

uniform sampler2D u_texture;

void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0); //texture2D(u_texture, v_uv);
}