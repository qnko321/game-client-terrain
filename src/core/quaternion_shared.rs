
use nalgebra_glm as glm;

pub(crate) fn quat_from_euler_vec3(angles: glm::Vec3) -> glm::Quat {
    let cr = (angles.x * 0.5).cos();
    let sr = (angles.x * 0.5).sin();
    let cp = (angles.y * 0.5).cos();
    let sp = (angles.y * 0.5).sin();
    let cy = (angles.z * 0.5).cos();
    let sy = (angles.z * 0.5).sin();

    glm::Quat::new(
        sr * cp * cy - cr * sp * sy,
        cr * sp * cy + sr * cp * sy,
        cr * cp * sy - sr * sp * cy,
        cr * cp * cy + sr * sp * sy,
    )
}