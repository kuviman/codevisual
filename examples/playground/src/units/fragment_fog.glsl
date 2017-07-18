varying vec2 v_v;

#define MID 0.7

void main() {
    float k;
    if (length(v_v) < MID) {
        k = 1.0;
    } else if (length(v_v) < 1.0) {
        k = (1.0 - length(v_v)) / (1.0 - MID);
    } else {
        k = 0.0;
    }
    gl_FragColor = vec4(1.0, 1.0, 1.0, k);
}