use nalgebra_glm as glm;

pub(crate) fn calculate_uv(
    texture_index: u16,
    uv: glm::Vec2,
    texture_atlas_size_in_blocks: u8,
    normalized_block_texture_size: f32,
) -> glm::Vec2 {
    let mut x_offset = texture_index as f32 / texture_atlas_size_in_blocks as f32;
    let contained = x_offset - x_offset % 1.0;
    x_offset -= contained;
    let mut y_offset: f32 = (texture_index as f32
        - texture_index as f32 % texture_atlas_size_in_blocks as f32)
        * (normalized_block_texture_size * normalized_block_texture_size) as f32;

    x_offset = x_offset + uv.x * normalized_block_texture_size;
    y_offset = y_offset + uv.y * normalized_block_texture_size;

    glm::vec2(x_offset, y_offset)
}
