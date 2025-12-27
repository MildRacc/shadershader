#version 330

in vec2 position;
out vec2 pos;

uniform float iTime;
uniform vec3 iResolution;

void main()
{
    pos = position;
    gl_Position = vec4(position, 0.0, 1.0);
}
