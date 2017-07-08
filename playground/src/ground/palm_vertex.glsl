attribute vec3 a_pos;
attribute vec2 a_vt;

attribute vec2 i_pos;
attribute float i_size;

varying vec2 v_vt;

uniform mat4 u_matrix;
uniform sampler2D u_map_texture;
uniform float u_map_size;

void main() {
    vec4 typ = texture2D(u_map_texture, i_pos / (2.0 * u_map_size) + vec2(0.5, 0.5));
    v_vt = a_vt;
    gl_Position = u_matrix * vec4(vec3(i_pos, -pow(typ.z, 1.5) * 2.0) + a_pos * i_size * 20.0, 1.0);
}