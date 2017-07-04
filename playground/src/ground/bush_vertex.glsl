attribute vec3 a_pos;
attribute vec2 a_vt;

attribute vec2 i_pos;

varying vec2 v_vt;

uniform mat4 u_matrix;

void main() {
    v_vt = a_vt;
    gl_Position = u_matrix * vec4(vec3(i_pos, 0.0) + a_pos * 3.0, 1.0);
}