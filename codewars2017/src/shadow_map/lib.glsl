#if shadows_enabled
uniform sampler2D u_shadow_map;

float get_shadow(vec3 pos) {
    float my_depth = 1.0 - pos.z / (2.0 * u_sky_height);
    pos -= u_light_direction * pos.z / u_light_direction.z;
    vec4 screen_pos = camera_matrix() * vec4(pos, 1.0);
    float depth = texture2D(u_shadow_map, screen_pos.xy / screen_pos.w / 2.0 + 0.5).x;

    if (depth < 1.0 && my_depth > depth) {
        float diff = my_depth - depth;
        float off = 1e-2;
        return 1.0 - min(diff / off, 1.0);
    } else {
        return 1.0;
    }
}
#else
float get_shadow(vec3 pos) {
    return 1.0;
}
#endif