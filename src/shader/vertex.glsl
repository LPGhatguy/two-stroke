#version 330

uniform Locals {
	mat4 u_ModelView;
	mat4 u_Projection;
	vec3 u_Color;
};

in vec3 a_Pos;
in vec3 a_Color;

out vec4 v_Color;

void main() {
	v_Color = vec4(u_Color * a_Color, 1.0);
	gl_Position = u_Projection * u_ModelView * vec4(a_Pos, 1.0);
}