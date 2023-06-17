use std::hash::{Hash, Hasher};
use crate::terrain::constants::CHUNK_SIZE;

#[derive(Clone, Debug, Copy)]
pub(crate) struct ChunkCoord {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl PartialEq<Self> for ChunkCoord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for ChunkCoord {}

impl Hash for ChunkCoord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl ChunkCoord {
    pub(crate) fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub(crate) fn from_world_coords(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: if x < 0 {
                x / CHUNK_SIZE as i32 - 1
            } else {
                x / CHUNK_SIZE as i32
            },
            y: if y < 0 {
                y / CHUNK_SIZE as i32 - 1
            } else {
                y / CHUNK_SIZE as i32
            },
            z: if z < 0 {
                z / CHUNK_SIZE as i32 - 1
            } else {
                z / CHUNK_SIZE as i32
            },
        }
    }

    pub(crate) fn add(&mut self, x: i32, y: i32, z: i32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub(crate) fn add_x(&mut self, x: i32) {
        self.x += x;
    }

    pub(crate) fn add_y(&mut self, y: i32) {
        self.y += y;
    }

    pub(crate) fn add_z(&mut self, z: i32) {
        self.z += z;
    }

    pub(crate) fn add_to_new(&self, x: i32, y: i32, z: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }

    pub(crate) fn add_x_to_new(&self, x: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
            z: self.z,
        }
    }

    pub(crate) fn add_y_to_new(&self, y: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
            z: self.z,
        }
    }

    pub(crate) fn add_z_to_new(&self, z: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + z,
        }
    }
}
