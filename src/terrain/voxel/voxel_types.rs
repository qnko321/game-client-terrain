use lazy_static::lazy_static;
use crate::terrain::direction_map::DirectionMap;
use crate::terrain::voxel::voxel_type::VoxelType;
use crate::terrain::voxel::voxel_face::VoxelFace;

lazy_static!(
    pub(crate) static ref VOXEL_TYPES : Vec<VoxelType> = {
        let mut types = Vec::new();
        // 0
        let air = VoxelType::new(vec![], false, DirectionMap::from_slice(&[true, true, true, true, true, true]));
        types.push(air);
        // 1
        let grass = VoxelType::new(
            vec![
                VoxelFace::front(2),
                VoxelFace::back(2),
                VoxelFace::left(2),
                VoxelFace::right(2),
                VoxelFace::top(7),
                VoxelFace::bottom(1),
            ],
            true,
            DirectionMap::from_slice(&[false, false, false, false, false, false])
        );
        types.push(grass);
        // 2
        let stone = VoxelType::new(
            vec![
                VoxelFace::front(0),
                VoxelFace::back(0),
                VoxelFace::left(0),
                VoxelFace::right(0),
                VoxelFace::top(0),
                VoxelFace::bottom(0),
            ],
            true,
            DirectionMap::from_slice(&[false, false, false, false, false, false])
        );
        types.push(stone);

        // 3
        let dirt = VoxelType::new(
            vec![
                VoxelFace::front(1),
                VoxelFace::back(1),
                VoxelFace::left(1),
                VoxelFace::right(1),
                VoxelFace::top(1),
                VoxelFace::bottom(1),
            ],
            true,
            DirectionMap::from_slice(&[false, false, false, false, false, false])
        );
        types.push(dirt);

        // 4
        let bedrock = VoxelType::new(
            vec![
                VoxelFace::front(9),
                VoxelFace::back(9),
                VoxelFace::left(9),
                VoxelFace::right(9),
                VoxelFace::top(9),
                VoxelFace::bottom(9),
            ],
            true,
            DirectionMap::from_slice(&[false, false, false, false, false, false])
        );
        types.push(bedrock);

        types
    };
);