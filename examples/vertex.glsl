attribute vec2 a_pos;
attribute vec4 a_color;

varying vec4 v_color;

uniform vec2 u_pos;

void main() {
    v_color = a_color;
    gl_Position = vec4(a_pos + u_pos, 0.0, 1.0);
}