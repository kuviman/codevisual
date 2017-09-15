varying vec4 v_color;

#ifdef VERTEX
uniform vec2 map_size;
uniform mat4 minimatrix;
attribute vec2 i_pos;
attribute float i_radius;
attribute vec4 i_color;
void main() {
    v_color = i_color;
    gl_Position = minimatrix * vec4(i_pos / map_size * 2.0 - 1.0, 0.0, 1.0);
    gl_PointSize = i_radius;
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = v_color;
}
#endif