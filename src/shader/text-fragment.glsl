#version 330

uniform sampler2D t_Texture;

in vec2 u_Uv;

out vec4 Target0;

void main() {
	Target0 = texture(t_Texture, u_Uv);
}