attribute vec2 a_pos;

attribute vec2 i_start_pos;
attribute float i_start_time;
attribute vec2 i_speed;
attribute float i_size;
attribute vec4 i_color;

varying vec2 v_pos;
varying vec4 v_color;

uniform float u_time;

void main() {
    v_color = i_color;
    v_pos = a_pos;
    gl_Position = vec4(a_pos * i_size + i_start_pos + i_speed * (u_time - i_start_time), 0.0, 1.0);
}