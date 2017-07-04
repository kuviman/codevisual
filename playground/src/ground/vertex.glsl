attribute vec2 a_pos;

varying vec2 v_pos;

uniform sampler2D u_map_texture;
uniform float u_map_size;
uniform mat4 u_matrix;

void main() {
    vec4 typ = texture2D(u_map_texture, a_pos / (2.0 * u_map_size) + vec2(0.5, 0.5));
    v_pos = a_pos;
    gl_Position = u_matrix * vec4(a_pos, -pow(typ.z, 1.5) * 2.0, 1.0);
}