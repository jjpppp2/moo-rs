#version 450
precision mediump float;

layout(location = 0) in vec3 v_color;
layout(location = 1) out vec4 color;

void main() {
    color = vec4(v_color, 1.0);
}