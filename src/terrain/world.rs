use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::thread::{spawn, Thread};
use anyhow::anyhow;
use vulkanalia::{Device, Instance, vk};
use nalgebra_glm as glm;
use crate::core::app_data::AppData;
use crate::core::math_functions::{remap, translate};
use crate::graphics::texturing_shared::calculate_uv;
use crate::graphics::vertex::Vertex;
use crate::terrain::chunk::chunk_mesh::ChunkMesh;
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::direction_map::DirectionMap;
use crate::terrain::perlin_noise::perlin_noise2d;
use crate::terrain::voxel::voxel_face::VoxelFace;
use crate::terrain::voxel::voxel_face_direction::VoxelFaceDirection;
use crate::terrain::voxel::voxel_mesh::VoxelMesh;
use crate::terrain::voxel::voxel_position::{VoxelChunkPosition, VoxelPositionAddError};
use crate::terrain::voxel::voxel_type::VoxelType;
use crate::terrain::voxel::voxel_types::VOXEL_TYPES;

type VoxelId = u8;
type ChunkVoxelMap = [u8; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize];

const CHUNK_SIZE: u8 = 32;
const VOXELS_COUNT_IN_CHUNK: usize = CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize;
pub(crate) static TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 4;
pub(crate) static NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0 / TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;


#[derive(Debug)]
pub(crate) struct World {
    pub(crate) chunks: HashMap<ChunkCoord, ThreadedChunk>
}

impl World {
    pub(crate) fn load(start_position: glm::Vec3) -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub(crate) fn generate_chunk_voxel_map(&mut self, coord: &ChunkCoord) {
        let mut voxel_map: ChunkVoxelMap = [0u8; VOXELS_COUNT_IN_CHUNK];
        for x in 0..CHUNK_SIZE as u8 {
            for y in 0..CHUNK_SIZE as u8 {
                let world_x = coord.x * CHUNK_SIZE as i32;
                let world_y = coord.y * CHUNK_SIZE as i32;

                let noise_value = remap(
                    perlin_noise2d(world_x as f64 * 0.1, world_y as f64 * 0.1),
                    -1.0,
                    1.0,
                    0.0,
                    1.0,
                );

                for z in 0..CHUNK_SIZE as u8 {
                    let world_z = coord.z * CHUNK_SIZE as i32 + z as i32;
                    let voxel_id = Self::get_voxel(world_z, noise_value);
                    let voxel_index = VoxelChunkPosition::new(x, y, z).to_index();
                    voxel_map[voxel_index] = voxel_id;
                }
            }
        }
        let mut threaded_chunk = ThreadedChunk::new(coord);
        threaded_chunk.chunk.voxel_map = voxel_map;
        self.chunks.insert(coord.clone(), threaded_chunk);
    }

    // TODO: MAKE THAT FUNCTION WITH MULTIPLE PASSES AND WAY MORE COMPLEX
    fn get_voxel(z: i32, height: f64) -> VoxelId {
        if z == 0 {
            return 4;
        }

        let height_multiplier = 6.0;
        let solid_ground_height = 10.0;
        let terrain_height = (height * height_multiplier).floor() + solid_ground_height;
        let voxel_id: VoxelId;

        if z as f64 == terrain_height {
            voxel_id = 1;
        } else if z as f64 == terrain_height && z as f64 > terrain_height - 4.0 {
            voxel_id = 3;
        } else if z as f64 > terrain_height {
            voxel_id = 0;
        } else {
            voxel_id = 2;
        }

        voxel_id
    }

    pub(crate) fn mesh_chunk_sync(&mut self, coord: &ChunkCoord, instance: &Instance, device: &Device, data: &mut AppData) {
        let mut threaded_chunk = self.chunks.get_mut(coord).expect("Couldn't get chunk"); // fix: generate chunk instead of panic

        let own_voxel_map = threaded_chunk.chunk.voxel_map;
        let neighbour_voxel_maps = self.get_neighbour_voxel_maps(coord);

        let mut chunk_mesh = ChunkMesh::new();
        for voxel_index in 0_usize..VOXELS_COUNT_IN_CHUNK {
            let voxel_id = *own_voxel_map.get(voxel_index).unwrap();
            if voxel_id == 0 {
                continue;
            }
            let voxel_mesh = Self::mesh_voxel(voxel_index, voxel_id, Self::should_draw(voxel_index, &own_voxel_map, &neighbour_voxel_maps), chunk_mesh.get_vertex_index());
            chunk_mesh.add_voxel_mesh(voxel_mesh);
        }
        create_chunk_vertex_buffer(instance, device, data, self)?;
        create_chunk_index_buffer(instance, device, data, self)?;
        println!("{:?}", chunk_mesh);
    }

/*    pub(crate) fn mesh_chunk(&mut self, coord: &ChunkCoord) {
        let mut threaded_chunk = self.chunks.get_mut(coord).expect("Couldn't get chunk"); // fix: generate chunk instead of panic
        if threaded_chunk.in_use {
            threaded_chunk.stop_sender.as_ref().unwrap().send(()).unwrap();
            threaded_chunk.has_stopped_receiver.as_ref().unwrap().recv().unwrap();

            let own_voxel_map = self.chunks.get(coord).unwrap().get_voxels();
            let neighbour_voxel_maps = self.get_neighbour_voxel_maps(coord);

            let (stop_sender, stop_receiver) = tokio::sync::oneshot::channel::<()>();

            let handle = tokio::spawn(async move {
                let mut stop_receiver = Some(stop_receiver);
                let mut chunk_mesh = ChunkMesh::new();
                for voxel_index in 0_usize..VOXELS_COUNT_IN_CHUNK {
                    let voxel_id = *own_voxel_map.get(voxel_index).unwrap();
                    if voxel_id == 0 {
                        continue;
                    }
                    tokio::select! {
                        data = Self::mesh_voxel(voxel_index, voxel_id, Self::should_draw(voxel_index, &own_voxel_map, &neighbour_voxel_maps), chunk_mesh.get_vertex_index()) => {
                            chunk_mesh.add_voxel_mesh(data);
                        }
                        _ = stop_receiver.as_mut().unwrap() => {
                            println!("Task is stopping");
                            break;
                        }
                    }
                }
            });
        }
    }
*/
    fn should_draw(voxel_index: usize, own_voxel_map: &ChunkVoxelMap, neighbour_voxel_maps: &DirectionMap<ChunkVoxelMap>) -> DirectionMap<bool>{
        let voxel_position = VoxelChunkPosition::from_index(voxel_index);

        let should_draw_vec = VoxelFaceDirection::to_vec().iter().map(|direction| {
            Self::should_draw_face(voxel_position, direction, own_voxel_map, neighbour_voxel_maps)
        }).collect::<Vec<bool>>();

        DirectionMap::from_slice(should_draw_vec.as_slice())
    }

    fn should_draw_face(voxel_position: VoxelChunkPosition, direction: &VoxelFaceDirection, own_voxel_map: &ChunkVoxelMap, neighbour_voxel_maps: &DirectionMap<ChunkVoxelMap>) -> bool {
         match voxel_position.add_from_direction(direction) {
            Ok(position) => {
                let voxel_id = *own_voxel_map.get(position.to_index()).unwrap();
                let neighbour_voxel_type: &VoxelType = VOXEL_TYPES.get(voxel_id as usize).unwrap();
                let should_draw = *neighbour_voxel_type.draw_neighbours.get_ref_by_voxel_face_direction(&direction).unwrap();
                should_draw
            }
            Err(error) => match error {
                VoxelPositionAddError::OutOfUpperBound(_, position) => {
                    println!("{:?}", position);
                    let voxel_id = *neighbour_voxel_maps.get_ref_by_voxel_face_direction(&direction).unwrap().get(position.to_index()).unwrap();
                    let neighbour_voxel_type: &VoxelType = VOXEL_TYPES.get(voxel_id as usize).unwrap();
                    let should_draw = *neighbour_voxel_type.draw_neighbours.get_ref_by_voxel_face_direction(&direction).unwrap();
                    should_draw
                }
                _ => false
            }
        }
    }

    /*async*/ fn mesh_voxel(voxel_index: usize, voxel_id: VoxelId, should_draw: DirectionMap<bool>, chunk_vertex_index: u32) -> VoxelMesh {
        let mut voxel_mesh = VoxelMesh::new();

        let voxel_type: &VoxelType = VOXEL_TYPES.get(voxel_id as usize).unwrap();
        let mut local_vertex_index: usize = 0;
        let voxel_chunk_pos = VoxelChunkPosition::from_index(voxel_index);
        for face in &voxel_type.faces {
            if should_draw.get_ref_by_voxel_face_direction(&face.direction).is_some() {
                for vertex_data in &face.vertices {
                    let uv = calculate_uv(face.texture, vertex_data.1, TEXTURE_ATLAS_SIZE_IN_BLOCKS, NORMALIZED_BLOCK_TEXTURE_SIZE);
                    voxel_mesh.vertices.push(Vertex {
                        position: voxel_chunk_pos.to_vec3() + vertex_data.0,
                        uv,
                    });
                }
                for index in &face.indices {
                    voxel_mesh.indices.push(chunk_vertex_index + voxel_mesh.vertex_index + index);
                }
                voxel_mesh.vertex_index += face.indices.len() as u32
            }
        }

        voxel_mesh
    }

    fn get_neighbour_voxel_maps(&self, coord: &ChunkCoord) -> DirectionMap<ChunkVoxelMap> {
        let back_neighbour = match self.chunks.get(&coord.add_y_to_new(1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let front_neighbour = match self.chunks.get(&coord.add_y_to_new(-1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let left_neighbour = match self.chunks.get(&coord.add_x_to_new(-1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let right_neighbour = match self.chunks.get(&coord.add_x_to_new(1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let top_neighbour = match self.chunks.get(&coord.add_z_to_new(1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let bottom_neighbour = match self.chunks.get(&coord.add_z_to_new(-1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };
        let back_neighbour = match self.chunks.get(&coord.add_x_to_new(-1)) {
            None => [0; 32*32*32],
            Some(threaded_chunk) => threaded_chunk.get_voxels(),
        };

        DirectionMap {
            front: front_neighbour,
            back: back_neighbour,
            left: left_neighbour,
            right: right_neighbour,
            top: top_neighbour,
            bottom: bottom_neighbour,
        }
    }

    pub(crate) fn get_chunk_by_index(&self, index: usize) -> anyhow::Result<&ThreadedChunk> {
        let mut counter = 0_i32;
        for (coord, chunk) in &self.chunks {
            if counter == index as i32 {
                return Ok(chunk);
            }
            counter += 1;
        }

        Err(anyhow!("No chunk at index: {}", index))
    }

    pub(crate) fn destroy(&self, device: &Device) {

    }
}

#[derive(Debug)]
pub struct ThreadedChunk {
    in_use: bool,
    should_draw: bool,
    chunk: Chunk,
    stop_sender: Option<crossbeam::channel::Sender<()>>,
    has_stopped_receiver: Option<crossbeam::channel::Receiver<()>>,
    model_matrix: glm::Mat4,

    //Buffers
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
}

impl ThreadedChunk {
    fn new(coord: &ChunkCoord) -> Self {
        let model_matrix = translate(glm::vec3(
            (coord.x * CHUNK_SIZE as i32) as f32,
            (coord.y * CHUNK_SIZE as i32) as f32,
            (coord.z * CHUNK_SIZE as i32) as f32,
        ));
        Self {
            in_use: false,
            should_draw: true,
            chunk: Chunk::new(),
            stop_sender: None,
            has_stopped_receiver: None,
            model_matrix: modle_matrix,
            vertex_buffer: Default::default(),
            vertex_buffer_memory: Default::default(),
            index: Default::default(),
            index_buffer_memory: Default::default(),
        }
    }

    fn set_buffers(vertex_buffer: vk::Buffer, vertex_buffer_memory: vk::DeviceMemory, index_buffer: vk::Buffer, index_buffer_memory: vk::DeviceMemory) {

    }

    fn get_voxels(&self) -> ChunkVoxelMap {
        self.chunk.voxel_map.clone()
    }

    fn get_voxel(&self, index: usize) -> VoxelId {
        *self.chunk.voxel_map.get(index).unwrap()
    }

    pub(crate) fn get_model_matrix(&self) -> glm::Mat4 {
        self.model_matrix
    }
}

// only the World can use Chunk struct
#[derive(Clone, Debug)]
pub struct Chunk {
    voxel_map: ChunkVoxelMap,
}

impl Chunk {
    fn new() -> Self {
        Self {
            voxel_map: [0; 32*32*32],
        }
    }
}