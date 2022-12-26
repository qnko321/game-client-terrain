pub(crate) struct ModelData {
    index_offset: u32,
}

impl Default for ModelData {
    fn default() -> Self {
        Self {
            index_offset: 0,
        }
    }
}