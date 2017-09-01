uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;

mat4 camera_matrix() {
    return u_projection_matrix * u_view_matrix;
}