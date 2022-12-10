use nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct UniformBufferObject {
    pub(crate) view: glm::Mat4,
    pub(crate) proj: glm::Mat4,
}
