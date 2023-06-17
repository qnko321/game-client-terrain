use crate::graphics::vertex::Vertex;

#[derive(Default)]
pub(crate) struct MeshData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl MeshData {
    pub(crate) fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self{
        Self {
            vertices,
            indices
        }
    }
}