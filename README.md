# ShaderShader

A minimal live shader editor written in Rust using GLIUM.

## What is it?

ShaderShader is a simple shader playground similar to ShaderToy. It provides a full-screen canvas where you can write and test GLSL fragment shaders in real-time.

## Features

- Live shader reloading - changes appear as you write
- Automatic shader compilation
- Compatible with basic ShaderToy shaders that use `iTime` and `iResolution`
- No manual compilation needed

## Uniforms

ShaderShader provides two uniforms:

- `float iTime` - elapsed time in seconds
- `vec3 iResolution` - window resolution (width, height, aspect ratio)

## Usage

Any ShaderToy shader that only uses `iTime` and `iResolution` will work out of the box. Just drop your fragment shader code into ShaderShader and it will automatically compile and display.

## Requirements

- Rust
- GLIUM
