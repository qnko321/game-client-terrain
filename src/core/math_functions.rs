use nalgebra_glm as glm;

// Matrix

pub(crate) fn translate(pos: glm::Vec3) -> glm::Mat4 {
    glm::Mat4::new(
        1.0, 0.0, 0.0, pos.x, 0.0, 1.0, 0.0, pos.y, 0.0, 0.0, 1.0, pos.z, 0.0, 0.0, 0.0, 1.0,
    )
}

pub(crate) fn rotate_x(a: f32) -> glm::Mat4 {
    glm::Mat4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        a.cos(),
        a.sin(),
        0.0,
        0.0,
        -(a.sin()),
        a.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub(crate) fn rotate_y(a: f32) -> glm::Mat4 {
    glm::Mat4::new(
        a.cos(),
        0.0,
        a.sin(),
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -a.sin(),
        0.0,
        a.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub(crate) fn rotate_z(a: f32) -> glm::Mat4 {
    glm::Mat4::new(
        a.cos(),
        a.sin(),
        0.0,
        0.0,
        -(a.sin()),
        a.cos(),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub(crate) fn rotate(r: glm::Vec3) -> glm::Mat4 {
    let ca = r.x.cos();
    let sa = r.x.sin();
    let cb = r.y.cos();
    let sb = r.y.sin();
    let cg = r.z.cos();
    let sg = r.z.sin();

    glm::Mat4::new(
        cb * cg,
        ca * sg - sa * sb * cg,
        ca * sb * cg + sa * sg,
        0.0,
        -cb * cg,
        sa * sb * sg + ca * cg,
        sa * cg - ca * sb * sg,
        0.0,
        -sb,
        -sa * cb,
        ca * cb,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

pub(crate) fn scale(scale: glm::Vec3) -> glm::Mat4x4 {
    glm::Mat4x4::new(
        scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

// Value manipulation
pub(crate) fn remap(value: f64, source_min: f64, source_max: f64, dest_min: f64, dest_max: f64) -> f64 {
    dest_min + ((value - source_min) / (source_max - source_min)) * (dest_max - dest_min)
}