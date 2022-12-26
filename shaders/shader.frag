#version 450

layout(binding = 1) uniform sampler2D texSampler;

layout(push_constant) uniform PushConstants {
    layout(offset = 64) float opacity;
} pcs;

layout(location = 0) in vec2 fragUV;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(texture(texSampler, fragUV).rgb, pcs.opacity); //texture(tex, vec3(fragUV, pcs.opacity));
}