use crate::terrain::direction_map::DirectionMap;
use crate::terrain::voxel::voxel_face::VoxelFace;
use crate::terrain::voxel::voxel_face_direction::VoxelFaceDirection;

#[derive(Debug)]
pub(crate) struct VoxelType {
    pub(crate) faces: Vec<VoxelFace>,
    pub(crate) collidable: bool,
    // Front Back Left Right Top Bottom
    pub(crate) draw_neighbours: DirectionMap<bool>,
}

impl VoxelType {
    pub(crate) fn new(faces: Vec<VoxelFace>, collidable: bool, draw_neighbours: DirectionMap<bool>) -> Self {
        Self {
            faces,
            collidable,
            draw_neighbours,
        }
    }

    pub(crate) fn should_draw(&self, direction: &VoxelFaceDirection) -> bool {
        if direction == &VoxelFaceDirection::Other {
            return true;
        }
        *self.draw_neighbours.get_ref_by_voxel_face_direction(direction).unwrap()
    }
}

