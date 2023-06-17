#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VoxelFaceDirection {
    Front = 0,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Other,
}

impl VoxelFaceDirection {
    pub(crate) fn reverse(&self) -> Self {
        match self {
            VoxelFaceDirection::Front => VoxelFaceDirection::Back,
            VoxelFaceDirection::Back => VoxelFaceDirection::Front,
            VoxelFaceDirection::Left => VoxelFaceDirection::Right,
            VoxelFaceDirection::Right => VoxelFaceDirection::Left,
            VoxelFaceDirection::Top => VoxelFaceDirection::Bottom,
            VoxelFaceDirection::Bottom => VoxelFaceDirection::Top,
            VoxelFaceDirection::Other => VoxelFaceDirection::Other,
        }
    }

    pub(crate) fn to_vec() -> Vec<VoxelFaceDirection> {
        vec![
            VoxelFaceDirection::Front,
            VoxelFaceDirection::Back,
            VoxelFaceDirection::Left,
            VoxelFaceDirection::Right,
            VoxelFaceDirection::Top,
            VoxelFaceDirection::Bottom,
        ]
    }
}
