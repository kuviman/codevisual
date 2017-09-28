#include <noise2d>
#include <codewars>

varying float v_type;
varying float v_light;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 i_pos;
attribute vec3 a_vn;
attribute float i_type;
attribute float i_size;
attribute float i_rotation;
void main() {
    v_type = i_type;
    v_light = get_light(a_vn);
    float size = i_size + sin(u_current_time * 2.0 + snoise(i_pos.xy) * 10.0) * 0.4;
    float sn = sin(i_rotation);
    float cs = cos(i_rotation);
    vec3 pos = vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z);
    gl_Position = camera_matrix() * vec4(pos * size + i_pos + vec3(0.0, 0.0, u_sky_height), 1.0);
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