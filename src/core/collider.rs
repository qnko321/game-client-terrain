use nalgebra_glm as glm;

#[derive(Debug, Clone, Default)]
pub(crate) struct Collider {
    pub(crate) vertices: Vec<glm::Vec3>,
}

impl Collider {
    pub(crate) fn find_furthest_point(&self, direction: glm::Vec3) -> glm::Vec3 {
        let mut furthest = self.vertices[0];
        let mut max_distance = -f32::INFINITY;
        for vertex in &self.vertices {
            let distance = glm::dot(vertex, &direction);
            if distance > max_distance {
                max_distance = distance;
                furthest = *vertex;
            }
        }
        furthest
    }

    pub(crate) fn compensate_position(&self, position: glm::Vec3) -> Collider {
        let vertices = self
            .vertices
            .iter()
            .map(|vertex| vertex + position)
            .collect::<Vec<_>>();

        Collider { vertices }
    }
}
