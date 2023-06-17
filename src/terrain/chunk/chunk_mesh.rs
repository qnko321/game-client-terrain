use crate::graphics::vertex::Vertex;
use crate::terrain::voxel::voxel_mesh::VoxelMesh;

#[derive(Debug)]
pub(crate) struct ChunkMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_index: u32,
}

impl ChunkMesh {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            indices: vec![],
            vertex_index: 0,
        }
    }

    pub(crate) fn add_voxel_mesh(&mut self, voxel_mesh: VoxelMesh) {
        self.vertices.extend_from_slice(voxel_mesh.vertices.as_slice());
        self.indices.extend_from_slice(voxel_mesh.indices.as_slice());
        self.vertex_index += voxel_mesh.vertices.len() as u32;
    }

    pub(crate) fn get_vertex_index(&self) -> u32 {
        self.vertex_index
    }
}