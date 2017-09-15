varying vec2 v_pos;

#ifdef VERTEX
attribute vec2 a_pos;
uniform vec2 map_size;
uniform mat4 minimatrix;
void main() {
    v_pos = a_pos / map_size * 2.0 - 1.0;
    gl_Position = minimatrix * vec4(v_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    if (-1.0 <= v_pos.x && v_pos.x <= 1.0 && -1.0 <= v_pos.y && v_pos.y <= 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.5);
    } else {
        discard;
    }
}
#endif