#include <noise2d>
#include <codewars>

varying vec2 v_pos;
varying vec2 v_quad_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_pos = a_v;
    v_quad_pos = a_quad_pos;
    gl_Position = camera_matrix() * vec4(a_v, u_sky_height, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D weather_map;
void main() {
    vec3 typ = texture2D(weather_map, v_quad_pos).xyz;
    gl_FragColor =
        typ.y * vec4(0.5, 0.5, 0.5, 0.85) * (snoise(v_pos / 32.0 + u_current_time * vec2(1.0, 1.0) / 5.0) * 0.05 + 0.95) +
        typ.z * vec4(0.1, 0.1, 0.15, 0.95) * (snoise(v_pos / 16.0 + u_current_time * vec2(1.0, 1.0) / 2.5) * 0.05 + 0.95);
}
#endif