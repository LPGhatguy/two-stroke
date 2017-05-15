#version 330

layout (location = 0) in vec3 a_Pos;
layout (location = 1) in vec3 a_Color;

uniform Locals {
	mat4 u_Transform;
};

out vec4 v_Color;

void main() {
	v_Color = vec4(a_Color, 1.0);
	gl_Position = vec4(a_Pos, 1.0);
}