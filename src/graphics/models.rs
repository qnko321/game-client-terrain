use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use anyhow::{Result};
use nalgebra_glm as glm;

use crate::graphics::vertex::Vertex;
use crate::AppData;
use crate::terrain::chunk::Chunk;
use crate::terrain::voxel::{Face, VoxelType};
use crate::terrain::world::get_mesh;

pub(crate) fn load_model(data: &mut AppData, path: &str) -> Result<()> {
    let mut reader = BufReader::new(File::open(path)?);

    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions {
            triangulate: true,
            ..std::default::Default::default()
        },
        |_| Ok(std::default::Default::default()),
    )?;

    //let mut unique_vertices = HashMap::new();




    /*let block = VoxelType::new(
        Face::block_faces([0; 6]),
        true,
        [false, false, false, false, false, false]);

    let mut vertex_index = 0;

    for face in block.faces {
        println!("face");
        for position in face.vertices {
            println!("vertex");
            let vertex = Vertex::new(
                position,
                glm::vec2(0.0, 0.0)
            );
            data.vertices.push(vertex);
        }
        for index in face.indices {
            println!("index");
            data.indices.push(vertex_index + index);
        }
        vertex_index += 4;
    }*/

    /*for model in &models {
        for index in &model.mesh.indices {
            let pos_offset = (3 * index) as usize;
            let tex_coord_offset = (2 * index) as usize;

            let vertex = Vertex {
                pos: glm::vec3(
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                tex_coord: glm::vec2(
                    model.mesh.texcoords[tex_coord_offset],
                    1.0 - model.mesh.texcoords[tex_coord_offset + 1],
                )
            };

            if let Some(index) = unique_vertices.get(&vertex) {
                data.indices.push(*index as u32);
            } else {
                let index = data.vertices.len();
                unique_vertices.insert(vertex, index);
                data.vertices.push(vertex);
                data.indices.push(index as u32);
            }
        }
    }*/

    Ok(())
}
