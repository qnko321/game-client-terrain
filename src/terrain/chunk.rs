use std::borrow::Borrow;
use std::time::Instant;
use lazy_static::lazy_static;
use nalgebra_glm as glm;
use nalgebra_glm::round;
use noise::{NoiseFn, Perlin, PerlinSurflet, Seedable};
use crate::graphics::vertex::Vertex;
use crate::terrain::perlin_noise::perlin_noise2d;
use crate::terrain::voxel::{Face, to_voxel_index, VoxelType};
use crate::terrain::world::block_meshes;

pub(crate) struct Chunk {
    voxels: [u8; 32768],
    coord: ChunkCoord,
}

impl Chunk {
    pub(crate) const fn size() -> i32 {
        32
    }

    pub(crate) const fn voxels_len() -> i32 {
        Chunk::size() * Chunk::size() * Chunk::size()
    }

    fn get_x(&self) -> i32 {
        self.coord.x * Self::size()
    }

    fn get_y(&self) -> i32 {
        self.coord.y * Self::size()
    }

    fn get_z(&self) -> i32 {
        self.coord.z * Self::size()
    }

    pub(crate) fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            voxels: [0; Chunk::voxels_len() as usize],
            coord: ChunkCoord { x, y, z },
        }
    }

    pub(crate) fn generate(&mut self) {
        for x in 0..Chunk::size() as u8 {
            for y in 0..Chunk::size() as u8 {
                for z in 0..Chunk::size() as u8 {
                    let (world_x, world_y, world_z) = self.voxel_to_world_coord(x ,y ,z);
                    let mut noise_value = perlin_noise2d(world_x as f64 * 0.1, world_y as f64 * 0.1);
                    noise_value = remap(noise_value, -1.0, 1.0, 0.0, 1.0);
                    let voxel_id = Self::get_voxel(world_z, noise_value);

                    let voxel_index = to_voxel_index(x, y, z);
                    self.voxels[voxel_index as usize] = voxel_id;
                }
            }
        }
    }

    fn voxel_to_world_coord(&mut self, x: u8, y: u8, z: u8) -> (i32, i32, i32) {
        let world_x = self.get_x() + x as i32;
        let world_y = self.get_y() + y as i32;
        let world_z =  self.get_z() + z as i32;

        (world_x, world_y, world_z)
    }

    fn get_voxel(z: i32, height: f64) -> u8 {
        if z == 0 {
            return 4; // Bedrock
        }

        let height_multiplier = 6.0;
        let solid_ground_height = 10.0;
        let terrain_height = (height * height_multiplier).floor() + solid_ground_height;
        let mut voxel: u8;

        if z as f64 == terrain_height {
            voxel = 1 // Grass
        } else if z as f64 == terrain_height && z as f64 > terrain_height - 4.0 {
            voxel = 3; // dirt
        } else if z as f64 > terrain_height {
            return 0; // air
        } else {
            voxel = 2; // stone
        }

        voxel
    }

    // u8 has max value of 256 but the max value of a voxel coord is 32=2^5 (3*3=9 unused bits)
    fn get_voxel_id(&self, x: u8, y: u8, z: u8) -> u8 {
        let index = to_voxel_index(x, y, z);
        self.voxels[index as usize]
    }

    pub(crate) fn to_mesh_data(&self, ref voxel_types: &block_meshes, texture_atlas_size_in_blocks: u8, normalized_block_texture_size: f32) -> MeshData {
        let mut mesh_data = MeshData { vertices: Vec::new(), indices: Vec::new(), vertex_index: 0 };
        for x in 0..Chunk::size() as u8 {
            for y in 0..Chunk::size() as u8 {
                for z in 0..Chunk::size() as u8 {
                    let voxel_id = self.get_voxel_id(x, y, z);
                    mesh_data.add_voxel(x, y, z, self.get_x(), self.get_y(), self.get_z(), voxel_id, self.voxels, voxel_types, texture_atlas_size_in_blocks, normalized_block_texture_size);
                }
            }
        }
        mesh_data
    }

    pub(crate) fn set_voxel(&mut self, x: u8, y: u8, z: u8, voxel_id: u8) {
        let index = to_voxel_index(x, y, z);
        self.voxels[index as usize] = voxel_id;
    }
}

fn remap(
    value: f64,
    source_min: f64,
    source_max: f64,
    dest_min: f64,
    dest_max: f64,
) -> f64 {
    dest_min + ((value - source_min) / (source_max - source_min)) * (dest_max - dest_min)
}

pub(crate) struct ChunkCoord {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

pub(crate) struct MeshData {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    vertex_index: u32,
}

fn get_neighbour_voxel_position(x: u8, y: u8, z: u8, direction: FaceDirection) -> (u8,u8,u8) {
    match direction {
        FaceDirection::Back => {
            if x == 0 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX,u8::MAX,u8::MAX) }
            (x - 1, y, z)
        },
        FaceDirection::Front => {
            if i32::from(x) == Chunk::size() - 1 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX, u8::MAX, u8::MAX) }
            (x + 1, y, z)
        },
        FaceDirection::Left => {
            if y == 0 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX,u8::MAX,u8::MAX) }
            (x, y - 1, z)
        },
        FaceDirection::Right => {
            if i32::from(y) == Chunk::size() - 1 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX, u8::MAX, u8::MAX) }
            (x, y + 1, z)
        },
        FaceDirection::Bottom => {
            if z == 0 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX,u8::MAX,u8::MAX) }
            (x, y, z - 1)
        },
        FaceDirection::Top => {
            if i32::from(z) == Chunk::size() - 1 { /*Get id from world (voxel is in another chunk)*/ return (u8::MAX, u8::MAX, u8::MAX) }
            (x, y, z + 1)
        },
        FaceDirection::Other => (x, y, z)
    }
}

static mut AIR_COUNTER: u32 = 0;

impl MeshData {
    fn calculate_uv(texture_index: u16, uv: glm::Vec2, texture_atlas_size_in_blocks: u8, normalized_block_texture_size: f32) -> glm::Vec2 {
        //let x_offset = face.texture as f32 / texture_atlas_size_in_blocks as f32;
        //let y_offset = (face.texture / texture_atlas_size_in_blocks as u16 ) as f32 * normalized_block_texture_size;

        //let new_uv = glm::vec2((uv.x / texture_atlas_size_in_blocks as f32) + x_offset, uv.y / texture_atlas_size_in_blocks as f32 + y_offset);

        let mut x_offset = texture_index as f32 / texture_atlas_size_in_blocks as f32;
        let contained = x_offset - x_offset % 1.0;
        x_offset -= contained;
        let mut y_offset: f32 = (texture_index as f32 - texture_index as f32 % texture_atlas_size_in_blocks as f32) * (normalized_block_texture_size * normalized_block_texture_size) as f32;//contained * normalized_block_texture_size + (texture_index / texture_atlas_size_in_blocks as u16 ) as f32 * normalized_block_texture_size;

        x_offset = x_offset + uv.x * normalized_block_texture_size;
        y_offset = y_offset + uv.y * normalized_block_texture_size;

        glm::vec2(
            x_offset,
            y_offset,
        )
    }

    fn add_voxel(
        &mut self,
        x: u8,
        y: u8,
        z: u8,
        chunk_x: i32,
        chunk_y: i32,
        chunk_z: i32,
        voxel_id: u8,
        ref voxel_map: [u8; Chunk::voxels_len() as usize],
        ref voxel_types: &Vec<VoxelType>,
        texture_atlas_size_in_blocks: u8,
        normalized_block_texture_size: f32,
    ) {
        if voxel_id == 0 {
            unsafe { AIR_COUNTER += 1 }
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
                    continue
                }
            }
            for i in 0..face.vertices.len() {
                let position = *face.vertices.get(i).unwrap();
                let uv = *face.uvs.get(i).unwrap();



                let vertex_uv = MeshData::calculate_uv(face.texture, uv, texture_atlas_size_in_blocks, normalized_block_texture_size);

                let vertex_position = glm::vec3(
                    x as f32 + position.x + chunk_x as f32,
                    y as f32 + position.y + chunk_y as f32,
                    z as f32 + position.z + chunk_z as f32,
                );
                let vertex = Vertex::new(
                    vertex_position,
                    vertex_uv,
                );
                self.vertices.push(vertex);
            }
            for index in &face.indices {
                self.indices.push(self.vertex_index + index);
            }
            self.vertex_index += face.vertices.len() as u32;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum FaceDirection {
    Front = 0,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Other
}

impl FaceDirection {
    pub(crate) fn reverse_face_direction(&self) -> FaceDirection {
        match self {
            FaceDirection::Front => FaceDirection::Back,
            FaceDirection::Back => FaceDirection::Front,
            FaceDirection::Left => FaceDirection::Right,
            FaceDirection::Right => FaceDirection::Left,
            FaceDirection::Top => FaceDirection::Bottom,
            FaceDirection::Bottom => FaceDirection::Top,
            FaceDirection::Other => FaceDirection::Other,
        }
    }
}