attribute vec2 a_v;
attribute vec2 i_start_pos;
attribute float i_start_time;
attribute float i_finish_time;
attribute vec2 i_speed;

uniform float u_time;
varying vec2 v_v;

#define MAP_SIZE 1000.0
#define RAD 150.0

void main() {
    v_v = a_v;
    float passed_time = min(u_time, i_finish_time) - i_start_time;
    gl_Position = vec4((i_start_pos + i_speed * passed_time + a_v * RAD) / MAP_SIZE, 0.0, 1.0);
}