use lazy_static::lazy_static;
use crate::AppData;
use crate::graphics::vertex::Vertex;
use crate::terrain::chunk::{Chunk, MeshData};
use crate::terrain::voxel::VoxelType;
use crate::terrain::voxel::Face;

lazy_static!(
    pub(crate) static ref block_meshes : Vec<VoxelType> = {
        let mut types = Vec::new();
        // 0
        let air = VoxelType::new(vec![], false, [true, true, true, true, true, true]);
        types.push(air);
        // 1
        let grass = VoxelType::new(
            vec![
                Face::front(2),
                Face::back(2),
                Face::left(2),
                Face::right(2),
                Face::top(7),
                Face::bottom(1),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(grass);
        // 2
        let stone = VoxelType::new(
            vec![
                Face::front(0),
                Face::back(0),
                Face::left(0),
                Face::right(0),
                Face::top(0),
                Face::bottom(0),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(stone);

        // 3
        let dirt = VoxelType::new(
            vec![
                Face::front(1),
                Face::back(1),
                Face::left(1),
                Face::right(1),
                Face::top(1),
                Face::bottom(1),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(dirt);

        // 4
        let bedrock = VoxelType::new(
            vec![
                Face::front(9),
                Face::back(9),
                Face::left(9),
                Face::right(9),
                Face::top(9),
                Face::bottom(9),
            ],
            true,
            [false, false, false, false, false, false]
        );
        types.push(bedrock);

        types
    };
);

static VIEW_DISTANCE_IN_CHUNKS: u8 = 4;
static TEXTURE_ATLAS_SIZE_IN_BLOCKS: u8 = 4;
static NORMALIZED_BLOCK_TEXTURE_SIZE: f32 = 1.0 / TEXTURE_ATLAS_SIZE_IN_BLOCKS as f32;

pub(crate) fn generate_world(data: &mut AppData) {
    let mut chunk: Chunk = Chunk::new(0, 1, 0);
    chunk.generate();
    let mut mesh_data = get_mesh(chunk);

    data.vertices = mesh_data.vertices;
    data.indices = mesh_data.indices;
}

pub(crate) fn get_mesh(chunk: Chunk) -> MeshData {
    let data = chunk.to_mesh_data(&block_meshes, TEXTURE_ATLAS_SIZE_IN_BLOCKS, NORMALIZED_BLOCK_TEXTURE_SIZE);
    data
}