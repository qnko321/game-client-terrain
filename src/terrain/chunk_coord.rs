use std::hash::{Hash, Hasher};
use crate::terrain::chunk::Chunk;

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
    pub(crate) fn from_world_coords(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x / Chunk::size(),
            y: y / Chunk::size(),
            z: z / Chunk::size(),
        }
    }
}