#version 330

in vec2 position;
in vec3 color;
out vec3 vertex_color;

uniform mat4 transform;

void main()
{
    vertex_color = color;
    gl_Position = transform * vec4(position, 0.0, 1.0);
}
