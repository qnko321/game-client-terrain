use nalgebra_glm as glm;
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use vulkanalia::prelude::v1_0::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Vertex {
    pub(crate) position: glm::Vec3,
    pub(crate) uv: glm::Vec2,
}

impl Vertex {
    pub fn new(position: glm::Vec3, uv: glm::Vec2) -> Self {
        Self { position, uv }
    }

    pub(crate) fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub(crate) fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let position = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
            .build();
        let uv = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(size_of::<glm::Vec3>() as u32)
            .build();
        [position, uv]
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.uv == other.uv
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position[0].to_bits().hash(state);
        self.position[1].to_bits().hash(state);
        self.position[2].to_bits().hash(state);
        self.uv[0].to_bits().hash(state);
        self.uv[1].to_bits().hash(state);
    }
}
