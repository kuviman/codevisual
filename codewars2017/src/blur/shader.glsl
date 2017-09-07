varying vec2 v_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_pos = a_quad_pos;
    gl_Position = vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT

#define RADIUS 5
#define SIGMA 0.8

uniform sampler2D texture;
void main() {
    vec4 result = vec4(0.0);
    float sum = 0.0;
    for (int i = -RADIUS; i <= RADIUS; i++)
        for (int j = -RADIUS; j <= RADIUS; j++) {
            float g = G(vec2(i, j), SIGMA);
            sum += g;
            result += texture2D(texture, v_pos + vec2(i, j) / texture_size) * g;
        }
    gl_FragColor = result / sum;
}
#endif