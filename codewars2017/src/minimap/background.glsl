#ifdef VERTEX
attribute vec2 a_v;
uniform mat4 minimatrix;
void main() {
    gl_Position = minimatrix * vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.5);
}
#endif