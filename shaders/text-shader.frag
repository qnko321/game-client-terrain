#version 450

layout(binding = 0) uniform sampler2D textures;

layout(location = 0) in vec2 TexCoords;
layout(location = 0) out vec4 outColor;

void main()
{
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(textures, TexCoords).r);
//    outColor = vec4(1.0, 0.0, 0.0, 1.0) * sampled;

    /*if (sampled == vec4(1.0, 1.0, 1.0, 1.0)) {
        outColor = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
        outColor = vec4(0.0, 0.0, 5.0, 0.7);
    }*/

    outColor = sampled;
}