use voxel_face_direction::VoxelFaceDirection;
use nalgebra_glm as glm;
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::constants::CHUNK_SIZE;

pub mod voxel_types;
pub mod voxel_type;
pub mod voxel_face;
pub mod voxel_position;
pub mod voxel_face_direction;
pub mod voxel_mesh;

pub(crate) type VoxelId = u8;

#[derive(Debug)]
pub(crate) struct VoxelType {
    pub(crate) faces: Vec<Face>,
    pub(crate) collidable: bool,
    // Front Back Left Right Top Bottom
    pub(crate) draw_neighbours: [bool; 6],
}

impl VoxelType {
    pub(crate) fn new(faces: Vec<Face>, collidable: bool, draw_neighbours: [bool; 6]) -> Self {
        Self {
            faces,
            collidable,
            draw_neighbours,
        }
    }

    pub(crate) fn should_draw(&self, direction: VoxelFaceDirection) -> bool {
        if direction == VoxelFaceDirection::Other {
            return true;
        }
        self.draw_neighbours[direction as usize]
    }
}

#[derive(Debug)]
pub(crate) struct Face {
    pub(crate) direction: VoxelFaceDirection,
    pub(crate) vertices: Vec<glm::Vec3>,
    pub(crate) indices: Vec<u32>,
    pub(crate) uvs: Vec<glm::Vec2>,
    pub(crate) texture: u16,
}

impl Face {
    pub(crate) fn back(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Back,
            vertices: vec![
                glm::vec3(0.0, 0.0, 0.0),
                glm::vec3(0.0, 0.0, 1.0),
                glm::vec3(0.0, 1.0, 1.0),
                glm::vec3(0.0, 1.0, 0.0),
            ],
            indices: vec![0, 1, 2, 3, 0, 2],
            uvs: vec![
                glm::vec2(1.0, 1.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(0.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn front(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Front,
            vertices: vec![
                glm::vec3(1.0, 0.0, 0.0),
                glm::vec3(1.0, 0.0, 1.0),
                glm::vec3(1.0, 1.0, 1.0),
                glm::vec3(1.0, 1.0, 0.0),
            ],
            indices: vec![2, 1, 0, 2, 0, 3],
            uvs: vec![
                glm::vec2(0.0, 1.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(1.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn top(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Top,
            vertices: vec![
                glm::vec3(0.0, 0.0, 1.0),
                glm::vec3(1.0, 0.0, 1.0),
                glm::vec3(1.0, 1.0, 1.0),
                glm::vec3(0.0, 1.0, 1.0),
            ],
            indices: vec![0, 1, 3, 3, 1, 2],
            uvs: vec![
                glm::vec2(0.0, 1.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(1.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn bottom(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Bottom,
            vertices: vec![
                glm::vec3(0.0, 0.0, 0.0),
                glm::vec3(1.0, 0.0, 0.0),
                glm::vec3(1.0, 1.0, 0.0),
                glm::vec3(0.0, 1.0, 0.0),
            ],
            indices: vec![3, 1, 0, 2, 1, 3],
            uvs: vec![
                glm::vec2(0.0, 1.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(1.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn left(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Left,
            vertices: vec![
                glm::vec3(0.0, 0.0, 0.0),
                glm::vec3(0.0, 0.0, 1.0),
                glm::vec3(1.0, 0.0, 1.0),
                glm::vec3(1.0, 0.0, 0.0),
            ],
            indices: vec![2, 1, 0, 2, 0, 3],
            uvs: vec![
                glm::vec2(0.0, 1.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(1.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn right(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Right,
            vertices: vec![
                glm::vec3(0.0, 1.0, 0.0),
                glm::vec3(0.0, 1.0, 1.0),
                glm::vec3(1.0, 1.0, 1.0),
                glm::vec3(1.0, 1.0, 0.0),
            ],
            indices: vec![0, 1, 2, 3, 0, 2],
            uvs: vec![
                glm::vec2(1.0, 1.0),
                glm::vec2(1.0, 0.0),
                glm::vec2(0.0, 0.0),
                glm::vec2(0.0, 1.0),
            ],
            texture,
        }
    }

    pub(crate) fn block_faces(textures: [u16; 6]) -> Vec<Self> {
        vec![
            Self::front(textures[1]),
            Self::back(textures[2]),
            Self::left(textures[3]),
            Self::right(textures[4]),
            Self::top(textures[5]),
            Self::bottom(textures[6]),
        ]
    }
}

#[inline]
pub(crate) fn to_voxel_index(x: u8, y: u8, z: u8) -> u32 {
    x as u32 * CHUNK_SIZE as u32 * CHUNK_SIZE as u32 + y as u32 * CHUNK_SIZE as u32 + z as u32
}
/*
#[derive(Debug, Clone, Copy)]
pub(crate) struct VoxelChunkPosition {
    x: u8,
    y: u8,
    z: u8,
}

impl VoxelChunkPosition {
    pub(crate) fn new(x: u8, y: u8, z: u8) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub(crate) fn to_world_position(&self, chunk_coord: &ChunkCoord) -> VoxelWorldPosition {
        let world_x = chunk_coord.x * CHUNK_SIZE as i32 + self.x as i32;
        let world_y = chunk_coord.y * CHUNK_SIZE as i32 + self.y as i32;
        let world_z = chunk_coord.z * CHUNK_SIZE as i32 + self.z as i32;

        VoxelWorldPosition {
            x: world_x,
            y: world_y,
            z: world_z,
        }
    }

    pub(crate) fn to_index(&self) -> usize {
        self.x as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize + self.y as usize * CHUNK_SIZE as usize + self.z as usize
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self {
            z: (index % CHUNK_SIZE as usize) as u8,
            y: ((index / CHUNK_SIZE as usize) % CHUNK_SIZE as usize) as u8,
            x: (index / (CHUNK_SIZE as usize * CHUNK_SIZE as usize)) as u8
        }
    }

    pub(crate) fn to_vec3(&self) -> glm::Vec3 {
        glm::vec3(
            self.x as f32,
            self.y as f32,
            self.z as f32
        )
    }

    pub(crate) fn x(&self) -> u8 {
        self.x
    }

    pub(crate) fn y(&self) -> u8 {
        self.y
    }

    pub(crate) fn z(&self) -> u8 {
        self.z
    }
}

#[derive(Debug)]
pub(crate) struct VoxelWorldPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl VoxelWorldPosition {
    pub(crate) fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub(crate) fn to_chunk_position(&self) -> VoxelChunkPosition {
        let chunk_x = if self.x >= 0 {
            self.x % (CHUNK_SIZE as i32)
        } else {
            ((self.x % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32 - 1
        };
        let chunk_y = if self.y >= 0 {
            self.y % (CHUNK_SIZE as i32 - 1)
        } else {
            ((self.y % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32 - 1
        };
        let chunk_z = if self.z >= 0 {
            self.z % (CHUNK_SIZE as i32 - 1)
        } else {
            ((self.z % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) % CHUNK_SIZE as i32 - 1
        };
        VoxelChunkPosition {
            x: chunk_x as u8,
            y: chunk_y as u8,
            z: chunk_z as u8,
        }
    }

    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    pub(crate) fn y(&self) -> i32 {
        self.y
    }

    pub(crate) fn z(&self) -> i32 {
        self.z
    }

    pub(crate) fn get_chunk_coord(&self) -> ChunkCoord {
        ChunkCoord::from_world_coords(self.x, self.y, self.z)
    }
}
*/