#include <codewars>

varying vec2 v_pos;
varying vec2 v_quad_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_pos = a_v;
    v_quad_pos = a_quad_pos;
    gl_Position = camera_matrix() * vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D weather_map; // TODO: should be in separate lib
uniform sampler2D terrain_map;
uniform sampler2D plain_texture;
uniform sampler2D forest_texture;
uniform sampler2D swamp_texture;

void main() {
    vec3 typ = texture2D(terrain_map, v_quad_pos).xyz;

    vec2 cellPos = v_pos / u_cell_size;
    vec3 PLAIN_COLOR = texture2D(plain_texture, cellPos).xyz;
    vec3 FOREST_COLOR = texture2D(forest_texture, cellPos).xyz;
    vec3 SWAMP_COLOR = texture2D(swamp_texture, cellPos).xyz;

    gl_FragColor = vec4(PLAIN_COLOR * typ.x + FOREST_COLOR * typ.y + SWAMP_COLOR * typ.z, 1.0);

    vec3 weather_typ = texture2D(weather_map, v_quad_pos).xyz;
    gl_FragColor.xyz *= 1.0 - weather_typ.y * 0.5 - weather_typ.z * 0.8;
}
#endif