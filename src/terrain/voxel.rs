use nalgebra_glm as glm;
use crate::terrain::chunk::{Chunk, FaceDirection};

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

    pub(crate) fn should_draw(&self, direction: FaceDirection) -> bool {
        if direction == FaceDirection::Other { return true; }
        self.draw_neighbours[direction as usize]
    }
}

#[derive(Debug)]
pub(crate) struct Face {
    pub(crate) direction: FaceDirection,
    pub(crate) vertices: Vec<glm::Vec3>,
    pub(crate) indices: Vec<u32>,
    pub(crate) uvs: Vec<glm::Vec2>,
    pub(crate) texture: u16,
}

impl Face {
    pub(crate) fn back(texture: u16) -> Self {
        Self {
            direction: FaceDirection::Back,
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
            direction: FaceDirection::Front,
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
            direction: FaceDirection::Top,
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
            direction: FaceDirection::Bottom,
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
            direction: FaceDirection::Left,
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
            direction: FaceDirection::Right,
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

pub(crate) fn to_voxel_index(x: u8, y: u8, z: u8) -> u32 {
    /*x as u32 * 25 + y as u32 * 5 + z as u32*/
    x as u32 * Chunk::chunk_size() as u32 * Chunk::chunk_size() as u32
    +
    y as u32 * Chunk::chunk_size() as u32
    +
    z as u32
}