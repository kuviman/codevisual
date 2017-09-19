uniform mat4 u_eye_matrix;
uniform mat4 u_projection_matrix;

mat4 u_matrix() {
    return u_projection_matrix * u_eye_matrix;
}