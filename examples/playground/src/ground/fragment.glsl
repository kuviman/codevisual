varying vec2 v_pos;

uniform sampler2D u_grass_texture;
uniform sampler2D u_dirt_texture;
uniform sampler2D u_darkgrass_texture;
uniform sampler2D u_map_texture;
uniform sampler2D u_fog_map;
uniform float u_map_size;

void main() {
    vec4 typ = texture2D(u_map_texture, v_pos / (2.0 * u_map_size) + vec2(0.5, 0.5));
    gl_FragColor = texture2D(u_darkgrass_texture, v_pos / 10.0) * typ.x +
        texture2D(u_grass_texture, v_pos / 10.0) * typ.y +
        texture2D(u_dirt_texture, v_pos / 10.0) * typ.z;
    gl_FragColor *= texture2D(u_fog_map, v_pos / (2.0 * u_map_size) + vec2(0.5, 0.5)).x;
    gl_FragColor.w = 1.0;
}