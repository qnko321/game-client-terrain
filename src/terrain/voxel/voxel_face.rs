use crate::terrain::voxel::voxel_face_direction::VoxelFaceDirection;
use nalgebra_glm as glm;

#[derive(Debug)]
pub(crate) struct VoxelFace {
    pub(crate) direction: VoxelFaceDirection,
    pub(crate) vertices: Vec<(glm::Vec3, glm::Vec2)>,
    pub(crate) indices: Vec<u32>,
    pub(crate) texture: u16,
}

impl VoxelFace {
    pub(crate) fn back(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Back,
            vertices: vec![
                (glm::vec3(0.0, 0.0, 0.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(0.0, 0.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(0.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(0.0, 1.0, 0.0), glm::vec2(0.0, 1.0)),
            ],
            indices: vec![0, 1, 2, 3, 0, 2],
            texture,
        }
    }

    pub(crate) fn front(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Front,
            vertices: vec![
                (glm::vec3(1.0, 0.0, 0.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(1.0, 0.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(1.0, 1.0, 0.0), glm::vec2(1.0, 1.0)),
            ],
            indices: vec![2, 1, 0, 2, 0, 3],
            texture,
        }
    }

    pub(crate) fn top(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Top,
            vertices: vec![
                (glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(1.0, 0.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(0.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
            ],
            indices: vec![0, 1, 3, 3, 1, 2],
            texture,
        }
    }

    pub(crate) fn bottom(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Bottom,
            vertices: vec![
                (glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(1.0, 0.0, 0.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(1.0, 1.0, 0.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(0.0, 1.0, 0.0), glm::vec2(1.0, 1.0)),
            ],
            indices: vec![3, 1, 0, 2, 1, 3],
            texture,
        }
    }

    pub(crate) fn left(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Left,
            vertices: vec![
                (glm::vec3(0.0, 0.0, 0.0), glm::vec2(0.0, 1.0)),
                (glm::vec3(0.0, 0.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(1.0, 0.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(1.0, 0.0, 0.0), glm::vec2(1.0, 1.0)),
            ],
            indices: vec![2, 1, 0, 2, 0, 3],
            texture,
        }
    }

    pub(crate) fn right(texture: u16) -> Self {
        Self {
            direction: VoxelFaceDirection::Right,
            vertices: vec![
                (glm::vec3(0.0, 1.0, 0.0), glm::vec2(1.0, 1.0)),
                (glm::vec3(0.0, 1.0, 1.0), glm::vec2(1.0, 0.0)),
                (glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
                (glm::vec3(1.0, 1.0, 0.0), glm::vec2(0.0, 1.0)),
            ],
            indices: vec![0, 1, 2, 3, 0, 2],
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