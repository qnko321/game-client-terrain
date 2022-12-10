use nalgebra_glm as glm;
use nalgebra_glm::quat;
use crate::core::matrix_functions::*;
use crate::core::trigonometry_shared::FastConversions;

#[derive(Clone, Debug, Default)]
pub(crate) struct Transform {
    pub(crate) position: glm::Vec3,
    pub(crate) rotation: glm::Vec3,
    pub(crate) scale: glm::Vec3,
}

impl Transform {
    pub(crate) fn get_model_matrix(&self) -> glm::Mat4 {
        translate(self.position) * glm::quat_to_mat4(&self.euler_to_quat()) * scale(self.scale) //(rotate_z(self.rotation.z) * rotate_y(self.rotation.y) * rotate_x(self.rotation.x))        (rotate(self.rotation)
    }

    pub(crate) fn euler_to_quat(&self) -> glm::Qua<f32> {
        let x_radians = self.rotation.x.to_radians_fast();
        let y_radians = self.rotation.y.to_radians_fast();
        let z_radians = self.rotation.z.to_radians_fast();

        let cr = (x_radians * 0.5).cos();
        let sr = (x_radians * 0.5).sin();
        let cp = (y_radians * 0.5).cos();
        let sp = (y_radians * 0.5).sin();
        let cy = (z_radians * 0.5).cos();
        let sy = (z_radians * 0.5).sin();

        glm::Qua::new(
            cr * cp * cy + sr * sp * sy,
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
        )
    }

    pub(crate) fn forward(&self) -> glm::Vec3 {
        let quat = self.euler_to_quat();
        // i = x   j = y    k = z   w = w
        glm::vec3(
            2.0 * (quat.i * quat.k - quat.w * quat.j),
            2.0 * (quat.j * quat.k + quat.w * quat.i),
            1.0 - 2.0 * (quat.i * quat.i + quat.j * quat.j),
        )
    }

    pub(crate) fn right(&mut self) -> glm::Vec3 {
        glm::vec3(
            /*(self.horizontal_angle - 3.14 / 2.0).sin() as f32,
            (self.horizontal_angle - 3.14 / 2.0).cos() as f32,
            0.0,*/
            0.0,0.0,0.0
        )
    }
}