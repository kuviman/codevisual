precision mediump float;

varying vec2 v_pos;
varying vec4 v_color;

void main() {
    gl_FragColor = vec4(v_color.xyz, max(0.0, 1.0 - length(v_pos)));
}