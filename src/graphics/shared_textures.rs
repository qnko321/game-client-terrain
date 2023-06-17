use crate::graphics::shared_buffers::create_buffer;
use crate::graphics::shared_images::{
    copy_buffer_to_image, create_image, create_image_view, transition_image_layout,
};
use anyhow::Result;
use std::fs::File;
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;
use crate::core::app_data::AppData;

pub(crate) unsafe fn create_texture_image_view(
    device: &Device,
    image: &vk::Image,
) -> Result<vk::ImageView> {
    let image_view = create_image_view(
        device,
        *image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageAspectFlags::COLOR,
    )?;

    Ok(image_view)
}

pub(crate) unsafe fn create_texture_image_from_path(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    path: &str,
) -> Result<(vk::Image, vk::DeviceMemory)> {
    // Load

    let image_file = File::open(path)?;

    let decoder = png::Decoder::new(image_file);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let (width, height) = reader.info().size();

    let (image, image_memory) = create_texture_image_from_byte_buffer(
        instance,
        device,
        data,
        width,
        height,
        pixels.as_slice(),
    )?;
    Ok((image, image_memory))
}

pub(crate) unsafe fn create_texture_image_from_byte_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
    width: u32,
    height: u32,
    buffer: &[u8],
) -> Result<(vk::Image, vk::DeviceMemory)> {
    let size = buffer.len() as u64;

    // Create (staging)

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

    memcpy(buffer.as_ptr(), memory.cast(), buffer.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (image)

    let (texture_image, texture_image_memory) = create_image(
        instance,
        device,
        data,
        width,
        height,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    // Transition + Copy (image)

    transition_image_layout(
        device,
        data,
        texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    )?;

    copy_buffer_to_image(device, data, staging_buffer, texture_image, width, height)?;

    transition_image_layout(
        device,
        data,
        texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok((texture_image, texture_image_memory))
}
