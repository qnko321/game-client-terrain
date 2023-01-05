use crate::graphics::vertex::Vertex;
use crate::terrain::chunk::{Chunk};
use crate::terrain::face_direction::FaceDirection;
use crate::terrain::voxel::{to_voxel_index, VoxelType};
use nalgebra_glm as glm;
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::world::World;

#[derive(Clone, Debug)]
pub(crate) struct MeshData {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    pub(crate) vertex_index: u32,
}

impl Default for MeshData {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_index: 0,
        }
    }
}

impl MeshData {
    pub(crate) fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_index: 0,
        }
    }/*

    pub(crate) fn add_vertex(&mut self, position: glm::Vec3, uv: glm::Vec2) {
        let vertex = Vertex::new(position, uv);
        self.vertices.push(vertex);
    }

    pub(crate) fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    pub(crate) fn increment_vertex_index(&mut self, count: u32) {
        vertex_index =
    }*/
}

