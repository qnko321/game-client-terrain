use crate::core::transform::Transform;
use nalgebra_glm as glm;
use crate::core::collider::Collider;

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

impl PlayerData {
    pub(crate) fn get_collider(&self) -> Collider {
        self.collider.compensate_position(self.transform.position)
    }




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
}
/*
impl GameObject for PlayerData {
    fn create() -> Self {
        Self {
            transform: Transform {
                position: glm::Vec3::new(0.0, 0.0, 0.0),
                rotation: glm::Vec3::new(0.0, 0.0, 0.0),
                scale: glm::Vec3::new(0.0, 0.0, 0.0),
            },
            mouse_speed: 2.0,
            move_speed: 10.0,
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
        }
    }

    fn start(&mut self, app: &mut App) {

    }

    fn update(&mut self, app: &mut App) {
        if app.input_manager.get_key(VirtualKeyCode::W) {
            let mut forward = self.transform.forward();
            forward.z = 0.0;
            self.walk(forward, app.delta_time);
        }
        if app.input_manager.get_key(VirtualKeyCode::S) {
            let mut backward = -self.transform.forward();
            backward.z = 0.0;
            self.walk(backward, app.delta_time);
        }
        if app.input_manager.get_key(VirtualKeyCode::D) {
            let right = self.transform.right();
            self.walk(right, app.delta_time);
        }
        if app.input_manager.get_key(VirtualKeyCode::A) {
            let left = -self.transform.right();
            self.walk(left, app.delta_time);
        }
    }
}*/
