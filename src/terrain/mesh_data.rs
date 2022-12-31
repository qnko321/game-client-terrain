use crate::graphics::vertex::Vertex;
use crate::terrain::chunk::{Chunk, get_neighbour_voxel_position};
use crate::terrain::face_direction::FaceDirection;
use crate::terrain::voxel::{to_voxel_index, VoxelType};
use nalgebra_glm as glm;

#[derive(Clone, Debug)]
pub(crate) struct MeshData {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    vertex_index: u32,
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
    }

    fn calculate_uv(
        texture_index: u16,
        uv: glm::Vec2,
        texture_atlas_size_in_blocks: u8,
        normalized_block_texture_size: f32,
    ) -> glm::Vec2 {
        let mut x_offset = texture_index as f32 / texture_atlas_size_in_blocks as f32;
        let contained = x_offset - x_offset % 1.0;
        x_offset -= contained;
        let mut y_offset: f32 = (texture_index as f32
            - texture_index as f32 % texture_atlas_size_in_blocks as f32)
            * (normalized_block_texture_size * normalized_block_texture_size) as f32;

        x_offset = x_offset + uv.x * normalized_block_texture_size;
        y_offset = y_offset + uv.y * normalized_block_texture_size;

        glm::vec2(x_offset, y_offset)
    }

    pub(crate) fn add_voxel(
        &mut self,
        x: u8,
        y: u8,
        z: u8,
        voxel_id: u8,
        ref voxel_map: [u8; Chunk::voxels_len() as usize],
        ref voxel_types: &Vec<VoxelType>,
        texture_atlas_size_in_blocks: u8,
        normalized_block_texture_size: f32,
    ) {
        if voxel_id == 0 {
            return;
        }
        let voxel: &VoxelType = voxel_types.get(voxel_id as usize).unwrap();

        for face in &voxel.faces {
            if face.direction != FaceDirection::Other {
                let (n_x, n_y, n_z) = get_neighbour_voxel_position(x, y, z, face.direction.clone());
                let neighbour_voxel_id = if n_x == u8::MAX && n_y == u8::MAX && n_z == u8::MAX {
                    0
                } else {
                    voxel_map[to_voxel_index(n_x, n_y, n_z) as usize]
                };
                let voxel: &VoxelType = voxel_types.get(neighbour_voxel_id as usize).unwrap();
                if !voxel.should_draw(face.direction.reverse_face_direction()) {
                    continue;
                }
            }
            for i in 0..face.vertices.len() {
                let position = *face.vertices.get(i).unwrap();
                let uv = *face.uvs.get(i).unwrap();

                let vertex_uv = MeshData::calculate_uv(
                    face.texture,
                    uv,
                    texture_atlas_size_in_blocks,
                    normalized_block_texture_size,
                );

                let vertex_position = glm::vec3(
                    x as f32 + position.x,
                    y as f32 + position.y,
                    z as f32 + position.z,
                );
                let vertex = Vertex::new(vertex_position, vertex_uv);
                self.vertices.push(vertex);
            }
            for index in &face.indices {
                self.indices.push(self.vertex_index + index);
            }
            self.vertex_index += face.vertices.len() as u32;
        }
    }
}