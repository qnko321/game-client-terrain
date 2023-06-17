use crate::graphics::shared_buffers::{copy_buffer, copy_buffer_offset, create_buffer};
use crate::graphics::text_textures::Character;
use crate::graphics::uniform_buffer_object::UniformBufferObject;
use crate::graphics::vertex::Vertex;
use anyhow::{anyhow, Result};
use log::error;
use nalgebra_glm as glm;
use std::collections::HashMap;
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{Buffer, DeviceMemory, DeviceSize};
use crate::core::app_data::AppData;

pub(crate) unsafe fn create_chunk_vertex_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    chunk: &mut Chunk,
) -> Result<()> {
    if chunk.get_mesh().vertices.len() == 0 {
        error!("No Vertices => Can't create a chunk vertex buffer");
    }

    // Create (staging)

    let vertices_size = size_of::<Vertex>() * chunk.get_mesh().vertices.len();
    let size = (vertices_size + CHUNK_VERTEX_BUFFER_FREE_SPACE) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    // Copy (staging)

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(
        chunk.get_mesh().vertices.as_ptr(),
        memory.cast(),
        chunk.get_mesh().vertices.len(),
    );

    device.unmap_memory(staging_buffer_memory);

    // Create (vertex)

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    chunk.set_vertex_buffer(vertex_buffer);
    chunk.set_vertex_buffer_memory(vertex_buffer_memory);

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    chunk
        .get_vertex_buffer_manager_mut()
        .add_free_region(vertices_size, CHUNK_VERTEX_BUFFER_FREE_SPACE);

    Ok(())
}

pub(crate) unsafe fn create_chunk_index_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    chunk: &mut Chunk,
) -> Result<()> {
    let indices_size = size_of::<u32>() * chunk.get_mesh().indices.len();
    let size = (indices_size + CHUNK_INDEX_BUFFER_FREE_SPACE) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(
        chunk.get_mesh().indices.as_ptr(),
        memory.cast(),
        chunk.get_mesh().indices.len(),
    );

    device.unmap_memory(staging_buffer_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    chunk.set_index_buffer(index_buffer);
    chunk.set_index_buffer_memory(index_buffer_memory);

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    chunk
        .get_index_buffer_manager_mut()
        .add_free_region(indices_size, CHUNK_INDEX_BUFFER_FREE_SPACE);

    println!("created new index buffer");

    Ok(())
}

pub(crate) unsafe fn update_buffer_region(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    buffer: Buffer,
    new_data: *const u8,
    size: usize,  // size in bytes
    count: usize, // size in object size * instances
    offset: usize,
) -> Result<()> {
    if size < 1 {
        return Err(anyhow!("No data to create buffer"));
    }

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size as DeviceSize,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size as DeviceSize,
        vk::MemoryMapFlags::empty(),
    )?;

    println!("size {}", size);

    memcpy(new_data, memory.cast(), count);

    device.unmap_memory(staging_buffer_memory);

    copy_buffer_offset(
        device,
        data,
        staging_buffer,
        buffer,
        size as DeviceSize,
        offset as DeviceSize,
    )?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}


pub(crate) unsafe fn update_buffer_region_vertex(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    buffer: Buffer,
    new_data: *const Vertex,
    size: usize,  // size in bytes
    count: usize, // size in object size * instances
    offset: usize,
) -> Result<()> {
    if size < 1 {
        return Err(anyhow!("No data to create buffer"));
    }

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size as DeviceSize,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size as DeviceSize,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(new_data, memory.cast(), count);

    device.unmap_memory(staging_buffer_memory);

    copy_buffer_offset(
        device,
        data,
        staging_buffer,
        buffer,
        size as DeviceSize,
        offset as DeviceSize,
    )?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}


pub(crate) unsafe fn update_buffer_region_u32(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    buffer: Buffer,
    new_data: *const u32,
    size: usize,  // size in bytes
    count: usize, // size in object size * instances
    offset: usize,
) -> Result<()> {
    if size < 1 {
        return Err(anyhow!("No data to create buffer"));
    }

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size as DeviceSize,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size as DeviceSize,
        vk::MemoryMapFlags::empty(),
    )?;

    println!("size {}", size);

    memcpy(new_data, memory.cast(), count);

    device.unmap_memory(staging_buffer_memory);

    copy_buffer_offset(
        device,
        data,
        staging_buffer,
        buffer,
        size as DeviceSize,
        offset as DeviceSize,
    )?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}


pub(crate) unsafe fn create_uniform_buffers(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}

pub(crate) unsafe fn create_text_vertex_index_buffers_multi(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let start_timer = Instant::now();
    let mut vertices: Arc<Mutex<Vec<glm::TVec4<f32>>>> = Arc::new(Mutex::new(vec![]));
    let mut indices: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![]));
    let (window_size_x, window_size_y) = (
        data.swapchain_extent.width as f32,
        data.swapchain_extent.height as f32,
    );

    let available_text_objects: Vec<usize> = (0..data.text_objects.len()).collect();

    let characters_clone = Arc::new(data.text_characters.clone());

    let mut vertex_index: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    crossbeam::thread::scope(|s| {
        for index in available_text_objects {
            let text_object = data.text_objects.get(index).unwrap();
            let characters_clone_arc = Arc::clone(&characters_clone);
            let vertices_clone = Arc::clone(&vertices);
            let indices_clone = Arc::clone(&indices);
            let vertex_index_clone = Arc::clone(&vertex_index);

            s.spawn(move |_| {
                let offset = glm::vec2(
                    2.0 * text_object.get_position().x / 1920.0 - 1.0,
                    2.0 * text_object.get_position().y / 1080.0 - 1.0,
                );

                let mut advance: f32 = 0.0;
                for char_index in text_object.get_chars() {
                    if *char_index == 32u32 {
                        advance += characters_clone_arc.get(&32).unwrap().advance as f32;
                    } else {
                        let character = characters_clone_arc.get(char_index).unwrap();
                        let width = character.size.x * text_object.get_scale();
                        let height = character.size.y * text_object.get_scale();

                        let normalized_advance = advance * text_object.get_scale();

                        let x_position = normalized_advance
                            + (character.bearing.x) * text_object.get_scale()
                            + offset.x;
                        let y_position = offset.y
                            - ((character.size.y - character.bearing.y) * text_object.get_scale());

                        let (x_start, x_end, y_start, y_end) =
                            character.texture_coordinates.unwrap();

                        let mut vertices_clone = vertices_clone.lock().unwrap();
                        let mut indices_clone = indices_clone.lock().unwrap();
                        let mut vertex_index_clone = vertex_index_clone.lock().unwrap();

                        vertices_clone.push(glm::vec4(
                            x_position,
                            y_position + height,
                            x_start,
                            y_end,
                        ));
                        vertices_clone.push(glm::vec4(
                            x_position + width,
                            y_position,
                            x_end,
                            y_start,
                        ));
                        vertices_clone.push(glm::vec4(x_position, y_position, x_start, y_start));
                        vertices_clone.push(glm::vec4(
                            x_position + width,
                            y_position + height,
                            x_end,
                            y_end,
                        ));

                        indices_clone.extend_from_slice(&[
                            *vertex_index_clone,
                            *vertex_index_clone + 1,
                            *vertex_index_clone + 2,
                            *vertex_index_clone,
                            *vertex_index_clone + 3,
                            *vertex_index_clone + 1,
                        ]);
                        *vertex_index_clone += 4;

                        advance += character.advance as f32;
                    }
                }
            });
        }
    })
    .unwrap();

    let indices_count = indices.lock().unwrap().to_vec().len();
    let (vertex_buffer, vertex_buffer_memory) =
        create_text_vertex_buffer(vertices.lock().unwrap().to_vec(), instance, device, data)?;
    let (index_buffer, index_buffer_memory) =
        create_text_index_buffer(indices.lock().unwrap().to_vec(), instance, device, data)?;

    println!("{}", start_timer.elapsed().as_micros());

    /*data.text_vertex_buffer = vertex_buffer;
    data.text_vertex_buffer_memory = vertex_buffer_memory;

    data.text_index_buffer_length = indices_count;
    data.text_index_buffer = index_buffer;
    data.text_index_buffer_memory = index_buffer_memory;*/

    Ok(())
}

pub(crate) unsafe fn create_text_vertex_index_buffers(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let mut vertices: Vec<glm::TVec4<f32>> = vec![];
    let mut indices: Vec<u32> = vec![];
    let (window_size_x, window_size_y) = (
        data.swapchain_extent.width as f32,
        data.swapchain_extent.height as f32,
    );

    let mut vertex_index: u32 = 0;
    for text_object in &data.text_objects {
        let offset = glm::vec2(
            2.0 * (text_object.get_position().x) / (1920.0 - 0.0) - 1.0,
            2.0 * (text_object.get_position().y) / (1080.0 - 0.0) - 1.0,
        );

        let line_gap = data.font_data.line_gap + data.font_data.global_bounding_box.y_max;

        let mut advance: f32 = 0.0;
        let mut line_offset: f32 = 0.0;
        let mut character_index = 0;
        'character_loop: while character_index < text_object.get_chars().len() {
            let character: &u32 = text_object.get_chars().get(character_index).unwrap();
            if *character == 32u32 {
                advance += data.text_characters.get(&32).unwrap().advance as f32;
            } else {
                let character_data = data.text_characters.get(character).unwrap();
                let width = (character_data.bounding_box.x_max - character_data.bounding_box.x_min)
                    as f32
                    * text_object.get_scale(); //character_data.size.x * text_object.get_scale();
                let height = (character_data.bounding_box.y_max - character_data.bounding_box.y_min)
                    as f32
                    * text_object.get_scale(); //character_data.size.y * text_object.get_scale();

                /*let mut x_position = (advance + character_data.bearing.x) * text_object.get_scale() + offset.x;
                let mut y_position = line_offset + offset.y - ((character_data.size.y - character_data.bearing.y + character_data.y_min as f32) * text_object.get_scale());*/

                let mut x_position = (advance + character_data.bounding_box.x_min as f32)
                    * text_object.get_scale()
                    + offset.x;
                let mut y_position = line_offset + offset.y
                    - character_data.bounding_box.y_max as f32 * text_object.get_scale();

                let (x_start, x_end, y_start, y_end) = character_data.texture_coordinates.unwrap();

                if text_object.get_wrap() && x_position + width > 1.0 {
                    advance = 0.0;
                    line_offset += line_gap as f32 * text_object.get_scale();
                    for i in (0..character_index).rev() {
                        if text_object.get_chars().get(i).unwrap() != &32u32 {
                            continue;
                        }
                        let go_back = character_index - i;
                        vertices.truncate(vertices.len() - (go_back - 1) * 4);
                        indices.truncate(indices.len() - (go_back - 1) * 6);
                        vertex_index -= (go_back - 1) as u32 * 4;
                        character_index = i + 1;
                        advance = 0.0;
                        continue 'character_loop;
                    }
                }

                vertices.push(glm::vec4(x_position, y_position + height, x_start, y_end));
                vertices.push(glm::vec4(x_position + width, y_position, x_end, y_start));
                vertices.push(glm::vec4(x_position, y_position, x_start, y_start));
                vertices.push(glm::vec4(
                    x_position + width,
                    y_position + height,
                    x_end,
                    y_end,
                ));

                indices.extend_from_slice(&[
                    vertex_index,
                    vertex_index + 1,
                    vertex_index + 2,
                    vertex_index,
                    vertex_index + 3,
                    vertex_index + 1,
                ]);
                vertex_index += 4;
                advance += character_data.advance as f32;
            }

            character_index += 1;
        }
    }

    if vertices.len() == 0 || indices.len() == 0 {
        return Ok(());
    }

    let indices_count = indices.len();
    let (vertex_buffer, vertex_buffer_memory) =
        create_text_vertex_buffer(vertices, instance, device, data)?;
    let (index_buffer, index_buffer_memory) =
        create_text_index_buffer(indices, instance, device, data)?;

    data.text_vertex_buffer = vertex_buffer;
    data.text_vertex_buffer_memory = vertex_buffer_memory;

    data.text_index_buffer_length = indices_count;
    data.text_index_buffer = index_buffer;
    data.text_index_buffer_memory = index_buffer_memory;

    Ok(())
}

unsafe fn create_text_vertex_buffer(
    vertices: Vec<glm::TVec4<f32>>,
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<(Buffer, DeviceMemory)> {
    if vertices.len() == 0 {
        return Err(anyhow!("No vertices to create text buffer"));
    }
    let size = (size_of::<glm::TVec4<f32>>() * vertices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(vertices.as_ptr(), memory.cast(), vertices.len());

    device.unmap_memory(staging_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok((vertex_buffer, vertex_buffer_memory))
}

unsafe fn create_text_index_buffer(
    indices: Vec<u32>,
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<(Buffer, DeviceMemory)> {
    let size = (size_of::<u32>() * indices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(indices.as_ptr(), memory.cast(), indices.len());

    device.unmap_memory(staging_buffer_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok((index_buffer, index_buffer_memory))
}
