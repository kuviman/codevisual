#include <global>
#include <format/obj>
#include <ground>
#include <units>

#define W 10.0

#ifdef VERTEX
attribute float i_size;
attribute vec4 i_color;
attribute float i_angle;
attribute float i_start_angle;

varying vec2 v_uv;
varying float v_light;

void main() {
    vec2 pos = unit_pos();
    float passed_time = min(u_time, i_finish_time) - i_start_time;
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
#if d_heightmap_enabled
    float mapheight = map_height(pos);
#else
    float mapheight = 0.0;
#endif
#if d_is_heli
    if (v.z > 1.5) {
        angle = u_time * 25.0;
        v = vec3(v.x * cos(angle) - v.y * sin(angle), v.x * sin(angle) + v.y * cos(angle), v.z);
        v.xy = v.xy * 2.0;
    }
    float height = max(0.0, mapheight) + 50.0;
#else
    float height = mapheight;
#endif
    gl_Position = u_matrix * vec4(v * i_size + vec3(pos, height), 1.0);
}
#endif

#ifdef FRAGMENT
varying vec2 v_uv;
varying float v_light;

uniform sampler2D u_texture;

void main() {
    vec4 texture = texture2D(u_texture, v_uv);
    if (texture.w < 0.5) {
        discard;
    }
    gl_FragColor = vec4(texture.xyz * v_light, 1.0);
}
#endif