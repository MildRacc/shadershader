#version 330

out vec4 color;
in vec3 vertex_color;

void main()
{
    color = vec4(vertex_color, 1.0);
}
