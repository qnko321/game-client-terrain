use crate::App;
use crate::core::transform::Transform;
use crate::graphics::model_data::ModelData;

pub(crate) struct GameObject {
    // Function calls
    priority: f32,
    start: fn(&mut Self),
    update: fn(&mut Self),
    // Transform
    transform: Transform,
    model_data: ModelData,
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            priority: 0.0,
            start: |_| {},
            update: |_| {},
            transform: Transform::default(),
            model_data: ModelData::default(),
        }
    }
}