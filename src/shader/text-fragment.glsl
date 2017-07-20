#version 330

uniform sampler2D texture_t;

in vec2 u_Uv;

out vec4 Target0;

void main() {
	Target0 = texture(texture_t, u_Uv);
}