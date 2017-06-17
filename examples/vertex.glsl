attribute vec3 a_v;
attribute vec3 a_n;
attribute vec2 a_vt;

attribute vec2 i_start_pos;
attribute float i_start_time;
attribute vec2 i_speed;
attribute float i_size;
attribute vec4 i_color;
attribute float i_angle;

varying vec2 v_uv;

uniform float u_time;
uniform mat4 u_matrix;
uniform float u_scale;
uniform vec2 u_pos;

void main() {
    v_uv = a_vt;
    vec3 v = vec3(a_v.x * cos(i_angle) - a_v.y * sin(i_angle), a_v.x * sin(i_angle) + a_v.y * cos(i_angle), a_v.z);
    gl_Position = u_matrix * vec4((v * i_size + vec3(u_pos + i_start_pos + i_speed * (u_time - i_start_time), 0.0)) * u_scale - vec3(0.0, 0.0, 0.5), 1.0);
}