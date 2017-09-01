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
const vec3 PLAIN_COLOR = vec3(1.0, 1.0, 1.0);
const vec3 FOREST_COLOR = vec3(0.0, 1.0, 0.0);
const vec3 SWAMP_COLOR = vec3(0.1, 0.7, 0.5);
void main() {
    vec3 typ = texture2D(texture, pos).xyz;
    gl_FragColor = vec4(PLAIN_COLOR * typ.x + FOREST_COLOR * typ.y + SWAMP_COLOR * typ.z, 1.0);
}
#endif