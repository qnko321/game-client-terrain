use nalgebra_glm as glm;
use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};
use std::ops;

#[derive(Clone)]
pub(crate) struct Simplex {
    points: [glm::Vec3; 4],
    size: u8,
}

impl Debug for Simplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Simplex")
            .field("points", &self.points)
            .field("size", &self.size)
            .finish()
    }
}

impl Simplex {
    pub(crate) fn empty() -> Self {
        Self {
            points: [glm::zero(); 4],
            size: 0,
        }
    }

    pub(crate) fn from_list(points: Vec<glm::Vec3>) -> Self {
        let mut new = Self {
            points: [glm::zero(); 4],
            size: points.len() as u8,
        };
        for i in 0..points.len() {
            new.points[i] = points[i];
        }
        new
    }

    pub(crate) fn push_front(&mut self, point: glm::Vec3) {
        self.points = [point, self.points[0], self.points[1], self.points[2]];
        self.size = min(self.size + 1, 4);
    }

    pub(crate) fn size(&self) -> u8 {
        self.size
    }

    pub(crate) fn get(&self, index: usize) -> &glm::Vec3 {
        self.points.get(index).unwrap()
    }

    pub(crate) fn set(&mut self, index: usize, value: glm::Vec3) {
        self.points[index] = value;
    }

    fn begin(&self) -> glm::Vec3 {
        self.points[0]
    }

    fn end(&self) -> glm::Vec3 {
        self.points[max(0, self.size - 1) as usize]
    }
}

impl ops::Index<u8> for Simplex {
    type Output = glm::Vec3;

    fn index(&self, index: u8) -> &Self::Output {
        &self.points[index as usize]
    }
}

impl ops::IndexMut<u8> for Simplex {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.points[index as usize]
    }
}
