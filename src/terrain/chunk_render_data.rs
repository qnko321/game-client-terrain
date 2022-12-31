/*use crate::core::matrix_functions::translate;
use crate::graphics::buffers::{create_chunk_index_buffer, create_chunk_vertex_buffer};
use crate::terrain::chunk::{Chunk, MeshData};
use crate::AppData;
use anyhow::Result;
use nalgebra_glm as glm;
use vulkanalia::vk::{DeviceV1_0, Handle};
use vulkanalia::{vk, Device, Instance};

#[derive(Clone, Debug)]
pub(crate) struct ChunkRenderData {
    //Model Matrix
    model_matrix: glm::Mat4,
    // State
    are_buffers_created: bool,
    // Vertices
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    // Indices
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    // Mesh
    mesh: MeshData,
}

impl Default for ChunkRenderData {
    fn default() -> Self {
        Self {
            model_matrix: glm::Mat4::identity(),
            are_buffers_created: false,
            vertex_buffer: vk::Buffer::default(),
            vertex_buffer_memory: vk::DeviceMemory::default(),
            index_buffer: vk::Buffer::default(),
            index_buffer_memory: vk::DeviceMemory::default(),
            mesh: MeshData::default(),
        }
    }
}

impl ChunkRenderData {
    pub(crate) fn new(x: i32, y: i32, z: i32, chunk: &Chunk) -> Self {
        let model_matrix = translate(glm::vec3(
            (x * Chunk::size()) as f32,
            (y * Chunk::size()) as f32,
            (z * Chunk::size()) as f32,
        ));
        Self {
            model_matrix,
            are_buffers_created: false,
            vertex_buffer: vk::Buffer::default(),
            vertex_buffer_memory: vk::DeviceMemory::default(),
            index_buffer: vk::Buffer::default(),
            index_buffer_memory: vk::DeviceMemory::default(),
            mesh: MeshData::default(),
        }
    }
}
*/