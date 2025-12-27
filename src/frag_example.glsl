#version 330

out vec4 col;
in vec2 pos;

uniform float iTime;
uniform vec3 iResolution;

void main()
{
    vec2 uv = pos * 0.5 + 0.5; 
    col = vec4(uv, 0.5 + 0.5 * sin(iTime), 1.0);
}
