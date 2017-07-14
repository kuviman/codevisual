varying vec2 v_uv;
varying float v_light;

uniform sampler2D u_car_texture;

void main() {
    gl_FragColor = vec4(texture2D(u_car_texture, v_uv).xyz * v_light, 1.0);
}