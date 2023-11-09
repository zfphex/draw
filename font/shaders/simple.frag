#version 330 core

in vec4 out_color;
in vec2 out_uv;

out vec4 color;

uniform sampler2D image;

void main() {
    color = out_color;
}