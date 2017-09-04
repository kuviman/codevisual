#include <codewars>

varying vec2 pos;

#ifdef VERTEX
attribute vec2 a_v;
void main() {
    pos = (a_v + 1.0) / 2.0;
    gl_Position = camera_matrix() * vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
uniform sampler2D plain_texture;
uniform sampler2D forest_texture;
uniform sampler2D swamp_texture;

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
    vec2 ppos = pos * texture_size;
    vec3 PLAIN_COLOR = texture2D(plain_texture, ppos).xyz;
    vec3 FOREST_COLOR = texture2D(forest_texture, ppos).xyz;
    vec3 SWAMP_COLOR = texture2D(swamp_texture, ppos).xyz;
    gl_FragColor = vec4(PLAIN_COLOR * typ.x + FOREST_COLOR * typ.y + SWAMP_COLOR * typ.z, 1.0);
}
#endif