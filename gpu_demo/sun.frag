#version 330 core

in vec3 object_position;
uniform float time;
out vec4 fragment_color;

void main() {
    vec3 p = object_position;
    float warp = sin(p.x * 11.0 + time) * sin(p.z * 13.0 - time * 0.7);
    float wave = sin(p.y * 24.0 + warp * 3.0 + time * 1.5) * 0.5 + 0.5;
    float cells = sin(p.x * 31.0 + p.z * 17.0 + time) * 0.5 + 0.5;
    float heat = pow(wave * 0.75 + cells * 0.25, 1.5);
    vec3 color = mix(vec3(0.45, 0.015, 0.002), vec3(1.0, 0.9, 0.25), heat);
    fragment_color = vec4(color, 1.0);
}
