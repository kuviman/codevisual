#include <codewars>

#define SHADOW_CAST_MATERIAL

#ifdef VERTEX
void set_shadow_pos(vec3 pos) {
    float depth = 1.0 - pos.z / (2.0 * u_sky_height);
    gl_Position = camera_matrix() * vec4(pos - u_light_direction * pos.z / u_light_direction.z, 1.0);
    if (gl_Position.z > 0.0) {
        gl_Position = vec4(gl_Position.xy / gl_Position.w, depth * 2.0 - 1.0, 1.0);
    }
}
#endif

#ifdef FRAGMENT
void main() {}
#define main unused_main
#endif