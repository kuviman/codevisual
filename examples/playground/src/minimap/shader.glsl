#include <global>
#include <units>

#ifdef VERTEX
uniform float point_size;
void main() {
    gl_Position = vec4(unit_pos() / u_map_size, 0.0, 1.0);
    gl_PointSize = point_size;
}
#endif

#ifdef FRAGMENT

uniform vec4 color;

void main() {
    gl_FragColor = vec4(color.xyz, color.w * pow(1.0 - length(gl_PointCoord.xy - 0.5) * 2.0, 0.5));
}
#endif