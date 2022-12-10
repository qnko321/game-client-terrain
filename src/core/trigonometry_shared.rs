impl FastConversions for f32 {
    fn to_radians_fast(self) -> f32 {
        self * 0.01745329251994329
    }

    fn to_degrees_fast(self) -> f32 {
        self * 57.29577951308232
    }
}

pub trait FastConversions {
    fn to_radians_fast(self) -> f32;
    fn to_degrees_fast(self) -> f32;
}