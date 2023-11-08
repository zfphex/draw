#version 330 core

layout(location = 0) in vec2 position; //TODO: Maybe combine position and uv?
layout(location = 1) in vec2 uv;
layout(location = 2) in vec4 color; //TODO: Maybe move into color?

uniform mat4 projection;

out vec4 out_color;
out vec2 out_uv;

void main() {
    gl_Position = projection * vec4(position, 0.0, 1.0);
    // gl_Position = vec4(position, 0.0, 1.0);
    out_color = color;
    out_uv = uv;
}