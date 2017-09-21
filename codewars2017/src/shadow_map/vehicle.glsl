#include <codewars>

varying float v_depth;

#ifdef VERTEX
attribute vec3 a_v;

attribute vec2 i_pos;
attribute float i_radius;
attribute float i_height;
attribute float i_angle;

void main() {
    float sn = sin(i_angle);
    float cs = cos(i_angle);
    vec3 pos = vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z) * i_radius +
        vec3(i_pos, i_height * u_sky_height);
    v_depth = 1.0 - pos.z / (2.0 * u_sky_height);
    gl_Position = camera_matrix() * vec4(pos - u_light_direction * pos.z / u_light_direction.z, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
#ifdef EMSCRIPTEN
    gl_FragDepthEXT = v_depth;
#else
    gl_FragDepth = v_depth;
#endif
}
#endif