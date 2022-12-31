use std::collections::HashMap;
use crate::terrain::chunk::{Chunk};
use crate::terrain::voxel::Face;
use crate::terrain::voxel::VoxelType;
use crate::AppData;
use lazy_static::lazy_static;
use vulkanalia::{Device, Instance};
use anyhow::{anyhow, Result};
use log::error;
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::mesh_data::MeshData;

lazy_static!(
    pub(crate) static ref BLOCK_MESHES : Vec<VoxelType> = {
        let mut types = Vec::new();
        // 0
        let air = VoxelType::new(vec![], false, [true, true, true, true, true, true]);
        types.push(air);
        // 1
        let grass = VoxelType::new(
            vec![
                Face::front(2),
                Face::back(2),
                Face::left(2),
                Face::right(2),
                Face::top(7),
                Face::bottom(1),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(grass);
        // 2
        let stone = VoxelType::new(
            vec![
                Face::front(0),
                Face::back(0),
                Face::left(0),
                Face::right(0),
                Face::top(0),
                Face::bottom(0),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(stone);

        // 3
        let dirt = VoxelType::new(
            vec![
                Face::front(1),
                Face::back(1),
                Face::left(1),
                Face::right(1),
                Face::top(1),
                Face::bottom(1),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(dirt);

        // 4
        let bedrock = VoxelType::new(
            vec![
                Face::front(9),
                Face::back(9),
                Face::left(9),
                Face::right(9),
                Face::top(9),
                Face::bottom(9),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(bedrock);

        types
    };
);

static VIEW_DISTANCE_IN_CHUNKS: i8 = 2;
static TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 4;
static NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0 / TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;

#[derive(Clone, Debug)]
pub(crate) struct World {
    chunks: HashMap<ChunkCoord, Chunk>,
    previously_active_chunks: Vec<ChunkCoord>,
    last_player_chunk: ChunkCoord,
}

impl World {
    pub(crate) const fn height_in_chunks() -> i32 {
        8
    }

    pub(crate) const fn height_in_voxels() -> i32 {
        Self::height_in_chunks() * Chunk::size()
    }

    pub(crate) fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            previously_active_chunks: Vec::new(),
            last_player_chunk: ChunkCoord::from_world_coords(0, 0, 0),
        }
    }

    pub(crate) unsafe fn load_spawn(&mut self, instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        for x in -VIEW_DISTANCE_IN_CHUNKS..VIEW_DISTANCE_IN_CHUNKS {
            for y in -VIEW_DISTANCE_IN_CHUNKS..VIEW_DISTANCE_IN_CHUNKS {
                let coord = ChunkCoord { x: x as i32, y: y as i32, z: 0 };
                let mut chunk = Chunk::from_coord(&coord);
                chunk.generate();
                let mesh_data = get_mesh(&chunk);
                chunk.update_mesh(mesh_data, instance, device, data)?;
                self.chunks.insert(coord, chunk);
                self.previously_active_chunks.push(coord);
            }
        }

        Ok(())
    }

    pub(crate) unsafe fn update_view_distance(&mut self, x: i32, y: i32, z: i32, instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
        let player_chunk = ChunkCoord::from_world_coords(x, y, z);
        if player_chunk == self.last_player_chunk {
            return Ok(());
        }
        self.last_player_chunk = player_chunk.clone();

        for previously_active_chunk in &self.previously_active_chunks {
            match self.chunks.get_mut(previously_active_chunk) {
                Some(chunk) => chunk.set_should_draw(false),
                None => { error!("Chunk coord is found in hashmap but can't retrieve it 1"); }
            }
        }
        self.previously_active_chunks.clear();

        for x_offset in -VIEW_DISTANCE_IN_CHUNKS..VIEW_DISTANCE_IN_CHUNKS {
            for y_offset in -VIEW_DISTANCE_IN_CHUNKS..VIEW_DISTANCE_IN_CHUNKS {
                // TODO: Add support for multiple chunks on the Z axis (height)
                let chunk_coord = ChunkCoord { x: player_chunk.x + x_offset as i32, y: player_chunk.y + y_offset as i32, z: 0 };
                if self.chunks.contains_key(&chunk_coord) {
                    match self.chunks.get_mut(&chunk_coord) {
                        Some(chunk) => {
                            chunk.set_should_draw(true);
                            self.previously_active_chunks.push(chunk_coord);
                        },
                        None => {
                            error!("Chunk coord is found in hashmap but can't retrieve it 2");
                        }
                    }
                } else {
                    let mut chunk = Chunk::from_coord(&chunk_coord);
                    chunk.generate();
                    let mesh_data = get_mesh(&chunk);
                    chunk.update_mesh(mesh_data, instance, device, data)?;
                    self.chunks.insert(chunk_coord, chunk);
                    self.previously_active_chunks.push(chunk_coord);
                }
            }
        }

        Ok(())
    }

    pub(crate) unsafe fn destroy(&mut self, device: &Device) {
        for chunk in self.chunks.values().into_iter() {
            chunk.destroy(device);
        }
    }

    pub(crate) fn chunks_len(&self) -> usize {
        self.chunks.len()
    }

    // TODO: Fix
    pub(crate) fn get_chunk_by_index(&self, index: usize) -> Result<&Chunk> {
        let mut counter = 0_i32;
        for (coord, chunk) in &self.chunks {
            if counter == index as i32 {
                return Ok(chunk);
            }
            counter += 1;
        }

        Err(anyhow!("No chunk at index: {}", index))
    }
}

pub(crate) fn get_mesh(chunk: &Chunk) -> MeshData {
    let data = chunk.to_mesh_data(
        &BLOCK_MESHES,
        TEXTURE_ATLAS_SIZE_IN_BLOCKS,
        NORMALIZED_BLOCK_TEXTURE_SIZE,
    );
    data
}
