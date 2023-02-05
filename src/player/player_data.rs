use std::any::Any;
use crate::core::transform::Transform;
use nalgebra_glm as glm;
use winit::event::VirtualKeyCode;
use crate::core::collider::Collider;
use crate::core::game_object::GameObject;
use crate::FrameData;
use crate::terrain::mesh_data::MeshData;

#[derive(Clone, Debug, Default)]
pub(crate) struct PlayerData {
    pub(crate) transform: Transform,
    pub(crate) collider: Collider,
    // Camera
    pub(crate) horizontal_angle: f32,
    pub(crate) vertical_angle: f32,
    pub(crate) mouse_speed: f32,

    // Movement
    pub(crate) move_speed: f32,
    pub(crate) velocity: glm::Vec3,
    pub(crate) is_grounded: bool,
}

impl GameObject for PlayerData {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_mesh(&self) -> MeshData {
        MeshData::default()
    }

    fn get_collider(&self) -> Collider {
        self.collider.compensate_position(self.transform.position)
    }

    fn start(&mut self, data: FrameData) {

    }

    fn update(&mut self, data: FrameData) {
        self.handle_camera(&data);
        self.handle_movement(&data);
    }
}

impl PlayerData {
    pub(crate) fn add_horizontal_angle(&mut self, add: f32) {
        self.horizontal_angle += add;
    }

    pub(crate) fn add_vertical_angle(&mut self, add: f32) {
        self.vertical_angle += add;
    }

    pub(crate) fn walk(&mut self, direction: glm::Vec3, delta_time: f32) {
        let new_pos = direction * self.move_speed * delta_time;
        self.transform.position += new_pos;
    }

    pub(crate) fn forward(&mut self) -> glm::Vec3 {
        glm::vec3(
            (self.vertical_angle.cos() * self.horizontal_angle.sin()) as f32,
            (self.vertical_angle.cos() * self.horizontal_angle.cos()) as f32,
            self.vertical_angle.sin() as f32,
        )
        .normalize()
    }

    pub(crate) fn right(&mut self) -> glm::Vec3 {
        glm::vec3(
            (self.horizontal_angle - 3.14 * 0.5).sin() as f32,
            (self.horizontal_angle - 3.14 * 0.5).cos() as f32,
            0.0,
        )
    }

    fn handle_camera(&mut self, data: &FrameData) {
        let (x_offset, y_offset) = data.input_manager.get_mouse_delta();

        self.horizontal_angle += data.delta_time
            * self.mouse_speed
            * x_offset as f32;
        self.vertical_angle += data.delta_time
            * self.mouse_speed
            * y_offset as f32;

        self.vertical_angle = glm::clamp_scalar(self.vertical_angle, 0.0 + 1.57, 6.28 - 1.57);
    }

    fn handle_movement(&mut self, data: &FrameData) {
        if data.input_manager.get_key(VirtualKeyCode::W) {
            let mut forward = self.forward();
            //forward.z = 0.0;
            self.walk(forward, data.delta_time);
        }
        if data.input_manager.get_key(VirtualKeyCode::S) {
            let mut backward = -self.forward();
            //backward.z = 0.0;
            self.walk(backward, data.delta_time);
        }
        if data.input_manager.get_key(VirtualKeyCode::D) {
            let right = self.right();
            self.walk(right, data.delta_time);
        }
        if data.input_manager.get_key(VirtualKeyCode::A) {
            let left = -self.right();
            self.walk(left, data.delta_time);
        }
    }
}
