#include <codewars>
#include <shadow>

#ifdef VERTEX
attribute vec3 a_v;

attribute vec2 i_pos;
attribute float i_size;
attribute float i_rotation;

void main() {
    float sn = sin(i_rotation);
    float cs = cos(i_rotation);
    vec3 pos = vec3(a_v.x * cs - a_v.y * sn, a_v.x * sn + a_v.y * cs, a_v.z) * i_size +
        vec3(i_pos, 0.0);
    float depth = 1.0 - pos.z / (2.0 * u_sky_height);
    gl_Position = camera_matrix() * vec4(pos - u_light_direction * pos.z / u_light_direction.z, 1.0);
    if (gl_Position.z > 0.0) {
        gl_Position = vec4(gl_Position.xy / gl_Position.w, depth * 2.0 - 1.0, 1.0);
    }
}
#endif

#ifdef FRAGMENT
void main() {}
#endif