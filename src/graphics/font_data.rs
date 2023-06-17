#[derive(Clone, Debug, Default)]
pub(crate) struct FontData {
    pub(crate) line_gap: i16,
    pub(crate) global_bounding_box: Rect,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rect {
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}

impl Rect {
    pub fn from_ttf_parser_rect(other: ttf_parser::Rect) -> Self {
        Self {
            x_min: other.x_min,
            y_min: other.y_min,
            x_max: other.x_max,
            y_max: other.y_max,
        }
    }

    #[inline]
    pub fn width(&self) -> i16 {
        self.x_max - self.x_min
    }

    #[inline]
    pub fn height(&self) -> i16 {
        self.y_max - self.y_min
    }
}
