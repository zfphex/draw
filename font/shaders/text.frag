#version 330 core

uniform sampler2D image;

out vec4 color;

in vec4 out_color;
in vec2 out_uv;

void main() {
    color = texture(image, out_uv).x * out_color;
}