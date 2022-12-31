use crate::graphics::shared_buffers::{copy_buffer, create_buffer};
use crate::graphics::uniform_buffer_object::UniformBufferObject;
use crate::graphics::vertex::Vertex;
use crate::terrain::chunk::{Chunk};
use crate::AppData;
use anyhow::Result;
use log::error;
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;

pub(crate) unsafe fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    if data.vertices.len() == 0 {
        error!("No Vertices => Can't create a vertex buffer");
    }

    // Create (staging)

    let size = (size_of::<Vertex>() * data.vertices.len()) as u64;

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

    memcpy(data.vertices.as_ptr(), memory.cast(), data.vertices.len());

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

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    // Copy (vertex)

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

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

    let size = (size_of::<Vertex>() * chunk.get_mesh().vertices.len()) as u64;

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

    // Copy (vertex)

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub(crate) unsafe fn create_index_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    // Create (staging)

    let size = (size_of::<u32>() * data.indices.len()) as u64;

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

    memcpy(data.indices.as_ptr(), memory.cast(), data.indices.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (index)

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.index_buffer = index_buffer;
    data.index_buffer_memory = index_buffer_memory;

    // Copy (index)

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub(crate) unsafe fn create_chunk_index_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    chunk: &mut Chunk,
) -> Result<()> {
    // Create (staging)

    let size = (size_of::<u32>() * chunk.get_mesh().indices.len()) as u64;

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
        chunk.get_mesh().indices.as_ptr(),
        memory.cast(),
        chunk.get_mesh().indices.len(),
    );

    device.unmap_memory(staging_buffer_memory);

    // Create (index)

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

    // Copy (index)

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    // Cleanup

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
