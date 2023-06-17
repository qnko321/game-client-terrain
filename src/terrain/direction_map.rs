use crate::terrain::voxel::voxel_face_direction::VoxelFaceDirection;

#[derive(Debug, Clone)]
pub(crate) struct DirectionMap<T: Clone> {
    pub(crate) front: T,
    pub(crate) back: T,
    pub(crate) left: T,
    pub(crate) right: T,
    pub(crate) top: T,
    pub(crate) bottom: T,
}

enum Directions {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom
}

impl<T: Clone> DirectionMap<T> {
    pub(crate) fn from_slice(data: &[T]) -> Self {
        Self {
            front: data[0].clone(),
            back: data[1].clone(),
            left: data[2].clone(),
            right: data[3].clone(),
            top: data[4].clone(),
            bottom: data[5].clone(),
        }
    }

    pub(crate) fn get_front(&self) -> T {
        self.front.clone()
    }
    pub(crate) fn get_back(&self) -> T {
        self.back.clone()
    }
    pub(crate) fn get_left(&self) -> T {
        self.left.clone()
    }
    pub(crate) fn get_right(&self) -> T {
        self.right.clone()
    }
    pub(crate) fn get_top(&self) -> T {
        self.top.clone()
    }
    pub(crate) fn get_bottom(&self) -> T {
        self.bottom.clone()
    }
    pub(crate) fn get_front_ref(&self) -> &T {
        &self.front
    }
    pub(crate) fn get_back_ref(&self) -> &T {
        &self.back
    }
    pub(crate) fn get_left_ref(&self) -> &T {
        &self.left
    }
    pub(crate) fn get_right_ref(&self) -> &T {
        &self.right
    }
    pub(crate) fn get_top_ref(&self) -> &T {
        &self.top
    }
    pub(crate) fn get_bottom_ref(&self) -> &T {
        &self.bottom
    }

    pub(crate) fn get_by_voxel_face_direction(&self, face_direction: VoxelFaceDirection) -> Option<T> {
        match face_direction {
            VoxelFaceDirection::Front => Some(self.front.clone()),
            VoxelFaceDirection::Back => Some(self.back.clone()),
            VoxelFaceDirection::Left => Some(self.left.clone()),
            VoxelFaceDirection::Right => Some(self.right.clone()),
            VoxelFaceDirection::Top => Some(self.top.clone()),
            VoxelFaceDirection::Bottom => Some(self.bottom.clone()),
            VoxelFaceDirection::Other => None
        }
    }
    pub(crate) fn get_ref_by_voxel_face_direction(&self, face_direction: &VoxelFaceDirection) -> Option<&T> {
        match face_direction {
            VoxelFaceDirection::Front => Some(&self.front),
            VoxelFaceDirection::Back => Some(&self.back),
            VoxelFaceDirection::Left => Some(&self.left),
            VoxelFaceDirection::Right => Some(&self.right),
            VoxelFaceDirection::Top => Some(&self.top),
            VoxelFaceDirection::Bottom => Some(&self.bottom),
            VoxelFaceDirection::Other => None
        }
    }
}