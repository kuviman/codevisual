attribute vec2 a_pos;

varying vec2 v_pos;
uniform mat4 u_matrix;

void main() {
    v_pos = a_pos;
    gl_Position = u_matrix * vec4(a_pos, -1.0, 1.0);
}