#include <noise2d>
#include <codewars>

varying vec2 v_pos;
varying float v_type;
varying float v_light;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 i_pos;
attribute vec3 a_n;
attribute float i_type;
attribute float i_size;
attribute vec3 a_cube_pos;
void main() {
    v_pos = i_pos.xy;
    v_type = i_type;
    v_light = max(0.0, dot(a_n, u_light_direction));
    float size = i_size + sin(u_current_time * 2.0 + snoise(i_pos.xy) * 10.0) * 0.4;
    gl_Position = camera_matrix() * vec4(a_v * size + i_pos + vec3(0.0, 0.0, u_sky_height), 1.0);
}
#endif

#ifdef FRAGMENT

#define CLOUD_COLOR vec3(1.0, 1.0, 1.0)
#define RAIN_COLOR vec3(0.5, 0.5, 0.7)

void main() {
    gl_FragColor = vec4((1.0 - v_type) * CLOUD_COLOR + v_type * RAIN_COLOR, 1.0);
    gl_FragColor.xyz *= v_light * 0.3 + 0.7;
}
#endif