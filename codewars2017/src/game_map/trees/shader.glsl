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
attribute float i_size;
attribute float i_rotation;

void main() {
    v_vt = a_vt;
    float sn = sin(i_rotation);
    float cs = cos(i_rotation);
    vec3 n = vec3(a_vn.x * cs - a_vn.y * sn, a_vn.x * sn + a_vn.y * cs, a_vn.z);
    v_light = get_light(n);
    v_pos = vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z) * i_size +
        vec3(i_pos, 0.0);
    gl_Position = camera_matrix() * vec4(v_pos, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
void main() {
    gl_FragColor = texture2D(texture, v_vt);
    gl_FragColor.xyz *= min(v_light, get_shadow(v_pos)) * (1.0 - u_ambient_light) + u_ambient_light;
}
#endif