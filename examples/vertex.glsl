attribute vec2 a_pos;
attribute vec4 a_color;

attribute vec2 i_pos;
attribute float i_speed;

varying vec4 v_color;

uniform float u_time;

void main() {
    v_color = a_color;
    float angle = u_time * i_speed;
    gl_Position = vec4(a_pos + i_pos + vec2(sin(angle), cos(angle)) * 0.05, 0.0, 1.0);
}