#include <global>
#include <ground>
#include <fog>

varying vec2 v_pos;

#ifdef VERTEX
attribute vec2 a_pos;

void main() {
    v_pos = a_pos;
#if d_heightmap_enabled
    float height = map_height(a_pos);
#else
    float height = 0.0;
#endif
    gl_Position = u_matrix * vec4(a_pos, height, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D u_grass_texture;
uniform sampler2D u_dirt_texture;
uniform sampler2D u_darkgrass_texture;

void main() {
    vec4 typ = map_type(v_pos);
    gl_FragColor = texture2D(u_darkgrass_texture, v_pos / 10.0) * typ.x +
        texture2D(u_grass_texture, v_pos / 10.0) * typ.y +
        texture2D(u_dirt_texture, v_pos / 10.0) * typ.z;
#if d_fog_enabled
    gl_FragColor.xyz *= fog_value(v_pos);
#endif
    gl_FragColor.w = 1.0;
}
#endif