attribute vec2 a_pos;

attribute vec2 i_start_pos;
attribute float i_start_time;
attribute vec2 i_speed;
attribute float i_size;
attribute vec4 i_color;

varying vec2 v_uv;

uniform float u_time;
uniform mat4 u_matrix;

void main() {
    v_uv = (a_pos + vec2(1.0, 1.0)) / 2.0;
    gl_Position = u_matrix * vec4(a_pos * i_size + i_start_pos + i_speed * (u_time - i_start_time), -1.0, 1.0);
}