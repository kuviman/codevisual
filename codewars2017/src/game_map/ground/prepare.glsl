varying vec2 v_quad_pos;

#ifdef VERTEX
attribute vec2 a_v;
attribute vec2 a_quad_pos;
void main() {
    v_quad_pos = a_quad_pos;
    gl_Position = vec4(a_v, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D texture;
void main() {
    vec3 typ = texture2D(texture, v_quad_pos).xyz;
    vec3 mx;
    if (typ.x > typ.y) {
        if (typ.x > typ.z)
            mx = vec3(1.0, 0.0, 0.0);
        else
            mx = vec3(0.0, 0.0, 1.0);
    } else {
        if (typ.y > typ.z)
            mx = vec3(0.0, 1.0, 0.0);
        else
            mx = vec3(0.0, 0.0, 1.0);
    }
    float diff = max(typ.x, max(typ.y, typ.z)) * 2.0 - 1.0;
    float k = max(0.0, min(diff * 4.0, 1.0));
    gl_FragColor = vec4(mx * k + typ * (1.0 - k), 1.0);
}
#endif