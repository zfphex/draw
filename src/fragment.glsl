#version 330 core
out vec4 frag_color;

uniform vec4 our_color;

void main() {
    // FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    frag_color = our_color;
}