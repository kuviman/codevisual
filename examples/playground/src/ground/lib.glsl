uniform sampler2D u_map_texture;

vec4 map_type(vec2 pos) {
    return texture2D(u_map_texture, pos / (2.0 * u_map_size) + vec2(0.5, 0.5));
}

float map_height(vec2 pos) {
    return -pow(map_type(pos).z, 1.5) * 7.0 + pow(map_type(pos).x, 1.5) * 100.0;
}