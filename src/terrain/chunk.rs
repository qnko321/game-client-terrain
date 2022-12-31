use crate::terrain::perlin_noise::perlin_noise2d;
use crate::terrain::voxel::{to_voxel_index};
use crate::terrain::world::{BLOCK_MESHES};
use crate::AppData;
use nalgebra_glm as glm;
use vulkanalia::{Device, Instance, vk};
use vulkanalia::vk::DeviceV1_0;
use anyhow::{Result};
use crate::core::math_functions::{remap, translate};
use crate::graphics::buffers::{create_chunk_index_buffer, create_chunk_vertex_buffer};
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::face_direction::FaceDirection;
use crate::terrain::mesh_data::MeshData;

#[derive(Clone, Debug)]
pub(crate) struct Chunk {
    // Data
    voxels: [u8; 32768],
    coord: ChunkCoord,

    // Render
    draw: bool,
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

impl Chunk {
    // Constants
    pub(crate) const fn size() -> i32 {
        32
    }

    pub(crate) const fn voxels_len() -> i32 {
        Chunk::size() * Chunk::size() * Chunk::size()
    }

    // Getters and setters
    fn get_x(&self) -> i32 {
        self.coord.x * Self::size()
    }

    fn get_y(&self) -> i32 {
        self.coord.y * Self::size()
    }

    fn get_z(&self) -> i32 {
        self.coord.z * Self::size()
    }

    pub(crate) fn should_draw(&self) -> bool {
        self.draw
    }

    pub(crate) fn set_should_draw(&mut self, value: bool) {
        self.draw = value;
    }

    pub(crate) fn get_model_matrix(&self) -> glm::Mat4 {
        self.model_matrix
    }

    pub(crate) fn get_mesh(&self) -> &MeshData {
        &self.mesh
    }

    pub(crate) fn set_vertex_buffer(&mut self, buffer: vk::Buffer) {
        self.vertex_buffer = buffer;
    }

    pub(crate) fn get_vertex_buffer(&self) -> vk::Buffer {
        self.vertex_buffer
    }

    pub(crate) fn set_vertex_buffer_memory(&mut self, memory: vk::DeviceMemory) {
        self.vertex_buffer_memory = memory;
    }

    pub(crate) fn set_index_buffer(&mut self, buffer: vk::Buffer) {
        self.index_buffer = buffer;
    }

    pub(crate) fn get_index_buffer(&self) -> vk::Buffer {
        self.index_buffer
    }

    pub(crate) fn set_index_buffer_memory(&mut self, memory: vk::DeviceMemory) {
        self.index_buffer_memory = memory;
    }

    // u8 has max value of 256 but the max value of a voxel coord is 32=2^5 (3*3=9 unused bits)
    fn get_voxel_id(&self, x: u8, y: u8, z: u8) -> u8 {
        let index = to_voxel_index(x, y, z);
        self.voxels[index as usize]
    }

    // Constructors

    pub(crate) fn from_xyz(x: i32, y: i32, z: i32) -> Self {
        let model_matrix = translate(glm::vec3(
            (x * Chunk::size()) as f32,
            (y * Chunk::size()) as f32,
            (z * Chunk::size()) as f32,
        ));
        Self {
            draw: true,
            voxels: [0; Chunk::voxels_len() as usize],
            coord: ChunkCoord { x, y, z },
            model_matrix,
            are_buffers_created: false,
            vertex_buffer: vk::Buffer::default(),
            vertex_buffer_memory: vk::DeviceMemory::default(),
            index_buffer: vk::Buffer::default(),
            index_buffer_memory: vk::DeviceMemory::default(),
            mesh: MeshData::default(),
        }
    }

    pub(crate) fn from_coord(coord: &ChunkCoord) -> Self {
        let model_matrix = translate(glm::vec3(
            (coord.x * Chunk::size()) as f32,
            (coord.y * Chunk::size()) as f32,
            (coord.z * Chunk::size()) as f32,
        ));
        Self {
            draw: true,
            voxels: [0; Chunk::voxels_len() as usize],
            coord: coord.clone(),
            model_matrix,
            are_buffers_created: false,
            vertex_buffer: vk::Buffer::default(),
            vertex_buffer_memory: vk::DeviceMemory::default(),
            index_buffer: vk::Buffer::default(),
            index_buffer_memory: vk::DeviceMemory::default(),
            mesh: MeshData::default(),
        }
    }

    // Conversions

    fn voxel_to_world_coord(&mut self, x: u8, y: u8, z: u8) -> (i32, i32, i32) {
        let world_x = self.get_x() + x as i32;
        let world_y = self.get_y() + y as i32;
        let world_z = self.get_z() + z as i32;

        (world_x, world_y, world_z)
    }

    // Generation

    pub(crate) fn generate(&mut self) {
        for x in 0..Chunk::size() as u8 {
            for y in 0..Chunk::size() as u8 {
                let world_x = self.get_x() + x as i32;
                let world_y = self.get_y() + y as i32;

                let noise_value = remap(
                    perlin_noise2d(world_x as f64 * 0.1, world_y as f64 * 0.1),
                    -1.0,
                    1.0,
                    0.0,
                    1.0
                );

                for z in 0..Chunk::size() as u8 {
                    let world_z = self.get_z() + z as i32;
                    let voxel_id = Self::get_voxel(world_z, noise_value);
                    let voxel_index = to_voxel_index(x, y, z);
                    self.voxels[voxel_index as usize] = voxel_id;
                }
            }
        }
    }

    fn get_voxel(z: i32, height: f64) -> u8 {
        if z == 0 {
            return 4; // Bedrock
        }

        let height_multiplier = 6.0;
        let solid_ground_height = 10.0;
        let terrain_height = (height * height_multiplier).floor() + solid_ground_height;
        let voxel: u8;

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

    // Mesh

    pub(crate) fn to_mesh_data(
        &self,
        ref voxel_types: &BLOCK_MESHES,
        texture_atlas_size_in_blocks: u8,
        normalized_block_texture_size: f32,
    ) -> MeshData {
        let mut mesh_data = MeshData::new();
        for x in 0..Chunk::size() as u8 {
            for y in 0..Chunk::size() as u8 {
                for z in 0..Chunk::size() as u8 {
                    let voxel_id = self.get_voxel_id(x, y, z);
                    mesh_data.add_voxel(
                        x,
                        y,
                        z,
                        voxel_id,
                        self.voxels,
                        voxel_types,
                        texture_atlas_size_in_blocks,
                        normalized_block_texture_size,
                    );
                }
            }
        }
        mesh_data
    }

    pub(crate) unsafe fn update_mesh(
        &mut self,
        new_mesh: MeshData,
        instance: &Instance,
        device: &Device,
        data: &mut AppData,
    ) -> Result<()> {
        self.mesh = new_mesh;
        self.recreate_buffers(instance, device, data)?;

        Ok(())
    }

    // Buffers

    unsafe fn recreate_buffers(
        &mut self,
        instance: &Instance,
        device: &Device,
        data: &mut AppData,
    ) -> Result<()> {
        if self.are_buffers_created {
            self.destroy_buffers(device);
        }

        self.create_buffers(instance, device, data)?;
        self.are_buffers_created = true;

        Ok(())
    }

    unsafe fn destroy_buffers(&self, device: &Device) {
        device.free_memory(self.index_buffer_memory, None);
        device.destroy_buffer(self.index_buffer, None);
        device.free_memory(self.vertex_buffer_memory, None);
        device.destroy_buffer(self.vertex_buffer, None);
    }

    unsafe fn create_buffers(
        &mut self,
        instance: &Instance,
        device: &Device,
        data: &mut AppData,
    ) -> Result<()> {
        create_chunk_vertex_buffer(instance, device, data, self)?;
        create_chunk_index_buffer(instance, device, data, self)?;

        Ok(())
    }

    pub(crate) unsafe fn destroy(&self, device: &Device) {
        self.destroy_buffers(device);
    }
}

pub(crate) fn get_neighbour_voxel_position(x: u8, y: u8, z: u8, direction: FaceDirection) -> (u8, u8, u8) {
    match direction {
        FaceDirection::Back => {
            if x == 0 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x - 1, y, z)
        }
        FaceDirection::Front => {
            if i32::from(x) == Chunk::size() - 1 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x + 1, y, z)
        }
        FaceDirection::Left => {
            if y == 0 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x, y - 1, z)
        }
        FaceDirection::Right => {
            if i32::from(y) == Chunk::size() - 1 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x, y + 1, z)
        }
        FaceDirection::Bottom => {
            if z == 0 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x, y, z - 1)
        }
        FaceDirection::Top => {
            if i32::from(z) == Chunk::size() - 1 {
                /*Get id from world (voxel is in another chunk)*/
                return (u8::MAX, u8::MAX, u8::MAX);
            }
            (x, y, z + 1)
        }
        FaceDirection::Other => (x, y, z),
    }
}
