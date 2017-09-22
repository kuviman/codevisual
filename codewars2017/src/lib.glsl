#include <camera>

uniform float u_sky_height;
uniform float u_current_time;
uniform float u_cell_size;

#define u_ambient_light float(0.3)
#define u_light_direction normalize(vec3(-3.0, 1.0, 7.0))

float get_light(vec3 n) {
    return max(0.0, dot(n, u_light_direction));
}