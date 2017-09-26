#include <codewars>
#include <shadow>

varying vec3 v_pos;
varying vec2 v_vt;
varying float v_light;

#ifdef VERTEX
attribute vec3 a_v;
attribute vec3 a_vn;
attribute vec2 a_vt;

attribute vec2 i_pos;
attribute float i_radius;
attribute float i_height;
attribute float i_angle;

void main() {
    v_vt = a_vt;
#ifdef HELICOPTER
    float i_angle = u_current_time * 100.0;
#endif
    float sn = sin(i_angle);
    float cs = cos(i_angle);
    vec3 n = vec3(a_vn.x * cs - a_vn.y * sn, a_vn.x * sn + a_vn.y * cs, a_vn.z);
    v_light = get_light(n);
    v_pos = vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z) * i_radius +
        vec3(i_pos, i_height * u_sky_height);
#ifdef SHADOW_CAST_MATERIAL
    set_shadow_pos(v_pos);
#else
    gl_Position = camera_matrix() * vec4(v_pos, 1.0);
#endif
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
void main() {
    gl_FragColor = texture2D(texture, v_vt);
    gl_FragColor.xyz *= min(v_light, get_shadow(v_pos)) * (1.0 - u_ambient_light) + u_ambient_light;
}
#endif