attribute vec3 a_v;
attribute vec3 a_vn;
attribute vec2 a_vt;

attribute vec2 i_start_pos;
attribute float i_start_time;
attribute vec2 i_speed;
attribute float i_size;
attribute vec4 i_color;
attribute float i_angle;
attribute float i_start_angle;

varying vec2 v_uv;
varying float v_light;

uniform float u_time;
uniform mat4 u_matrix;

#define W 10.0
#define PI 3.1415926535897932384626433832795

void main() {
    float passed_time = u_time - i_start_time;
    float angle_diff = i_angle - i_start_angle;
    if (angle_diff > PI) {
        angle_diff -= 2.0 * PI;
    }
    if (angle_diff < -PI) {
        angle_diff += 2.0 * PI;
    }
    float angle = i_start_angle + max(-W * passed_time, min(W * passed_time, angle_diff));
    v_uv = a_vt;
    vec3 n = vec3(a_vn.x * cos(angle) - a_vn.y * sin(angle), a_vn.x * sin(angle) + a_vn.y * cos(angle), a_vn.z);
    v_light = max(0.0, dot(normalize(n), normalize(vec3(3.0, 8.0, 10.0)))) * 0.7 + 0.3;
    vec3 v = vec3(a_v.x * cos(angle) - a_v.y * sin(angle), a_v.x * sin(angle) + a_v.y * cos(angle), a_v.z);
    gl_Position = u_matrix * vec4(v * i_size + vec3(i_start_pos + i_speed * passed_time, 0.0), 1.0);
}