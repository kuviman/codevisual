#include <noise2d>
#include <codewars>

varying vec2 pos;
uniform vec2 map_size;

#ifdef VERTEX
attribute vec2 a_v;
void main() {
    pos = (a_v + 1.0) / 2.0;
    gl_Position = camera_matrix() * vec4(pos * map_size, u_sky_height, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
vec4 blurred(sampler2D texture, vec2 pos) {
    vec4 result = vec4(0.0);
    float sum = 0.0;
    const int OFF = BLUR_RADIUS;
    for (int i = -OFF; i <= OFF; i++)
        for (int j = -OFF; j <= OFF; j++) {
            float g = G(vec2(i, j), BLUR_SIGMA);
            sum += g;
            result += texture2D(texture, pos + vec2(i, j) / texture_size / BLUR_DIV) * g;
        }
    return result / sum;
}

void main() {
//    gl_FragColor = vec4(G(pos * 2.0 - 1.0, 0.5), 0.0, 0.0, 1.0);
//    return;
#if BLUR
    vec3 typ = blurred(texture, pos).xyz;
#else
    vec3 typ = texture2D(texture, pos).xyz;
#endif
#if !VIEW_PLAIN
    if (typ.x > typ.y) {
        typ = typ.x > typ.z ? vec3(1.0, 0.0, 0.0) : vec3(0.0, 0.0, 1.0);
    } else {
        typ = typ.y > typ.z ? vec3(0.0, 1.0, 0.0) : vec3(0.0, 0.0, 1.0);
    }
#endif
//    typ.z = pow(typ.z, 0.3);
//    typ /= length(typ);
    gl_FragColor =
        typ.y * vec4(0.5, 0.5, 0.5, 0.85) * (snoise(pos * 32.0 + u_current_time * vec2(1.0, 1.0) / 5.0) * 0.05 + 0.95) +
        typ.z * vec4(0.1, 0.1, 0.15, 0.95) * (snoise(pos * 64.0 + u_current_time * vec2(1.0, 1.0) / 2.5) * 0.05 + 0.95);
}
#endif