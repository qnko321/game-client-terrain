use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::constants::CHUNK_SIZE;
use nalgebra_glm as glm;
use nalgebra_glm::abs;
use crate::terrain::voxel::voxel_face::VoxelFace;
use crate::terrain::voxel::voxel_face_direction::VoxelFaceDirection;

#[derive(Debug, Clone, Copy)]
pub(crate) struct VoxelChunkPosition {
    x: u8,
    y: u8,
    z: u8,
}

pub(crate) enum Bounds {
    X,
    Y,
    Z,
}

pub(crate) enum VoxelPositionAddError {
    OutOfLowerBound(Bounds, VoxelChunkPosition),
    OutOfUpperBound(Bounds, VoxelChunkPosition),
}

impl VoxelChunkPosition {
    pub(crate) fn new(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
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
        self.x as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize
            + self.y as usize * CHUNK_SIZE as usize
            + self.z as usize
    }

    pub(crate) fn from_index(index: usize) -> Self {
        Self {
            z: (index % CHUNK_SIZE as usize) as u8,
            y: ((index / CHUNK_SIZE as usize) % CHUNK_SIZE as usize) as u8,
            x: (index / (CHUNK_SIZE as usize * CHUNK_SIZE as usize)) as u8,
        }
    }

    pub(crate) fn to_vec3(&self) -> glm::Vec3 {
        glm::vec3(self.x as f32, self.y as f32, self.z as f32)
    }

    pub(crate) fn add_x(&self, x: i8) -> Result<Self, VoxelPositionAddError> {
        if self.x as i8 - x < 0 {
            return Err(VoxelPositionAddError::OutOfLowerBound(Bounds::X, Self {
                x: CHUNK_SIZE - ((self.x as i16 - x as i16).abs()) as u8,
                y: self.y,
                z: self.z,
            }));
        }

        let new_x = (self.x as i8 + x) as u8;

        if new_x >= CHUNK_SIZE {
            return Err(VoxelPositionAddError::OutOfUpperBound(Bounds::X, Self {
                x: new_x % CHUNK_SIZE,
                y: self.y,
                z: self.z,
            }));
        }

        Ok(Self {
            x: new_x,
            y: self.y,
            z: self.z,
        })
    }

    pub(crate) fn add_y(&self, y: i8) -> Result<Self, VoxelPositionAddError> {
        if self.y as i8 - y < 0 {
            return Err(VoxelPositionAddError::OutOfLowerBound(Bounds::Y, Self {
                x: self.x,
                y: CHUNK_SIZE - ((self.y as i16 - y as i16).abs()) as u8,
                z: self.z,
            }));
        }

        let new_y = (self.y as i8 + y) as u8;

        if new_y >= CHUNK_SIZE {
            return Err(VoxelPositionAddError::OutOfUpperBound(Bounds::Y, Self {
                x: self.x,
                y: new_y % CHUNK_SIZE,
                z: self.z,
            }));
        }

        Ok(Self {
            x: self.x,
            y: new_y,
            z: self.z,
        })
    }

    pub(crate) fn add_z(&self, z: i8) -> Result<Self, VoxelPositionAddError> {
        if self.z as i8 - z < 0 {
            return Err(VoxelPositionAddError::OutOfLowerBound(Bounds::Z, Self {
                x: self.x,
                y: self.y,
                z: CHUNK_SIZE - ((self.z as i16 - z as i16).abs()) as u8,
            }));
        }

        let new_z = (self.z as i8 + z) as u8;

        if new_z >= CHUNK_SIZE {
            return Err(VoxelPositionAddError::OutOfUpperBound(Bounds::Z, Self {
                x: self.x,
                y: self.y,
                z: new_z % CHUNK_SIZE,
            }));
        }

        Ok(Self {
            x: self.x,
            y: self.y,
            z: new_z,
        })
    }


    /*pub(crate) fn add_y(&self, y: i8) -> Result<Self, VoxelPositionAddError> {
        if (self.y as i8) < y {
            return Err(VoxelPositionAddError::OutOfLowerBound(Bounds::Y, Self {
                x: self.x,
                y: CHUNK_SIZE - ((self.y as i16 - y as i16).abs()) as u8,
                z: self.z,
            }));
        }

        let new_y = self.y + y as u8;

        if new_y >= CHUNK_SIZE {
            return Err(VoxelPositionAddError::OutOfUpperBound(Bounds::Y, Self {
                x: self.x,
                y: new_y % CHUNK_SIZE,
                z: self.z,
            }));
        }

        Ok(Self {
            x: self.x,
            y: new_y,
            z: self.z,
        })
    }

    pub(crate) fn add_z(&self, z: i8) -> Result<Self, VoxelPositionAddError> {
        if (self.z as i8) < z {
            return Err(VoxelPositionAddError::OutOfLowerBound(Bounds::Z, Self {
                x: self.x,
                y: self.y,
                z: CHUNK_SIZE - ((self.z as i16 - z as i16).abs()) as u8,
            }));
        }

        let new_z = ((self.z as i16 + z as i16) % CHUNK_SIZE as i16) as u8;

        if new_z >= CHUNK_SIZE {
            return Err(VoxelPositionAddError::OutOfUpperBound(Bounds::Z, Self {
                x: self.x,
                y: self.y,
                z: new_z % CHUNK_SIZE,
            }));
        }

        Ok(Self {
            x: self.z,
            y: self.y,
            z: new_z,
        })
    }*/

    pub(crate) fn add_from_direction(&self, direction: &VoxelFaceDirection) -> Result<Self, VoxelPositionAddError> {
        match direction {
            VoxelFaceDirection::Front => self.add_x(1),
            VoxelFaceDirection::Back => self.add_x(-1),
            VoxelFaceDirection::Left => self.add_y(-1),
            VoxelFaceDirection::Right => self.add_y(1),
            VoxelFaceDirection::Top => self.add_z(1),
            VoxelFaceDirection::Bottom => self.add_z(-1),
            VoxelFaceDirection::Other => Ok(*self),
        }
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
        Self { x, y, z }
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
