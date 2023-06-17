use crate::core::collider::Collider;
use crate::{FrameData};
use std::any::Any;
use vulkanalia::{Device, Instance};
use crate::terrain::mesh_data::MeshData;

pub(crate) trait GameObject {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_mesh(&self) -> MeshData;

    fn get_collider(&self) -> Collider;

    fn start(&mut self, data: FrameData);

    fn update(&mut self, data: FrameData);
}
