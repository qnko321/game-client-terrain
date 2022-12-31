#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum FaceDirection {
    Front = 0,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Other,
}

impl FaceDirection {
    pub(crate) fn reverse_face_direction(&self) -> FaceDirection {
        match self {
            FaceDirection::Front => FaceDirection::Back,
            FaceDirection::Back => FaceDirection::Front,
            FaceDirection::Left => FaceDirection::Right,
            FaceDirection::Right => FaceDirection::Left,
            FaceDirection::Top => FaceDirection::Bottom,
            FaceDirection::Bottom => FaceDirection::Top,
            FaceDirection::Other => FaceDirection::Other,
        }
    }
}