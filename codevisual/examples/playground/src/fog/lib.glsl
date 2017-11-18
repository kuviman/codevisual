#include <ground>

uniform sampler2D u_fog_map;

#define AMBIENT 0.5

float fog_value(vec2 pos) {
    float k = texture2D(u_fog_map, pos / (2.0 * u_map_size) + vec2(0.5, 0.5)).x;
    return AMBIENT + k * (1.0 - AMBIENT);
}