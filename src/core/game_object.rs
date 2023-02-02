use std::any::Any;
use crate::core::collider::Collider;
use crate::terrain::mesh_data::MeshData;

pub(crate) trait GameObject {
    fn as_any(&self) -> &dyn Any;

    fn get_mesh(&self) -> MeshData;

    fn get_collider(&self) -> Collider;
}