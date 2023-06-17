use crate::graphics::vertex::Vertex;

pub(crate) struct VoxelMesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertex_index: u32,
}

impl VoxelMesh {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            vertex_index: 0,
        }
    }
}