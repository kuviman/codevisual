#ifdef EMSCRIPTEN
precision highp float;
#endif

#define PI 3.1415926535897932384626433832795

float G(vec2 pos, float s) {
    float sq2 = 2 * s * s;
    return exp(-dot(pos, pos) / sq2) / (PI * sq2);
}