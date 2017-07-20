#version 330

in vec2 a_Pos;
in vec2 a_Uv;

out vec2 u_Uv;

void main() {
	u_Uv = a_Uv;
	gl_Position = vec4(a_Pos, 0.0, 1.0);
}