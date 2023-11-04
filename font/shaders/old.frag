#version 330 core

out vec4 color;
in vec4 out_color;

void main() {
    color = out_color;
}
