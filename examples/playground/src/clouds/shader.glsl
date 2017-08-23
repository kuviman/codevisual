#include <noise2d>
#include <global>

#ifdef VERTEX
attribute vec3 a_pos;

void main() {
    vec3 p = vec3(a_pos.xy, a_pos.z * 50.0 + 350.0);
    float TS = 0.3;
    float D = 35.0;
    p = vec3(
        p.x + sin(u_time * TS + snoise(p.xy) * PI) * D,
        p.y + sin(u_time * TS + snoise(p.xy + 5.0) * PI) * D,
        p.z + sin(u_time * TS + snoise(p.xy + 10.0) * PI) * D);
    gl_Position = u_matrix * vec4(p, 1.0);
    gl_PointSize = 7e4 / gl_Position.w;
}
#endif

#ifdef FRAGMENT

uniform sampler2D u_texture;

void main() {
    float k = texture2D(u_texture, gl_PointCoord).w * 0.2;
    float c = 0.2;
    gl_FragColor = vec4(c, c, c, k);
}
#endif