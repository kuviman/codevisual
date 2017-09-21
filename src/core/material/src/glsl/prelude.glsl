#ifdef EMSCRIPTEN
#if GL_EXT_frag_depth
#extension GL_EXT_frag_depth : enable
#endif
precision highp float;
#endif

#define PI 3.1415926535897932384626433832795

uniform vec2 u_framebuffer_size;

float G(vec2 pos, float s) {
    float sq2 = 2.0 * s * s;
    return exp(-dot(pos, pos) / sq2) / (PI * sq2);
}