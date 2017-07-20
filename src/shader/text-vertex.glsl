#version 330

uniform Locals {
	vec2 u_ScreenSize;
	vec2 u_TextureSize;
};

in vec2 a_Pos;
in vec2 a_Uv;

out vec2 u_Uv;

void main() {
	vec2 aspect = vec2(u_ScreenSize.y / u_ScreenSize.x, u_ScreenSize.x / u_ScreenSize.y);
	vec2 fraction = u_TextureSize / u_ScreenSize;

	u_Uv = a_Uv;
	gl_Position = vec4(aspect * fraction * a_Pos, 0.0, 1.0);
}