use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use crate::AppData;

pub(crate) unsafe fn create_command_buffers(device: &Device, data: &mut AppData) -> Result<()> {
    // Allocate

    let image_count = data.swapchain_images.len();
    for image_index in 0..image_count {
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(data.command_pools[image_index])
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = device.allocate_command_buffers(&allocate_info)?[0];
        data.command_buffers.push(command_buffer);
    }

    data.secondary_command_buffers = vec![vec![]; data.swapchain_images.len()];

    Ok(())
}
