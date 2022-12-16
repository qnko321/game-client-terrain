use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;
use std::time::Instant;

use anyhow::{anyhow, Result};
use log::*;
use nalgebra_glm as glm;
use png::Compression::Default;
use thiserror::Error;

use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::graphics::buffers::{create_index_buffer, create_uniform_buffers, create_vertex_buffer};
use crate::graphics::command_buffers::create_command_buffers;
use crate::graphics::command_pool::create_command_pools;
use crate::graphics::depth_objects::create_depth_objects;
use crate::graphics::descriptors::{create_descriptor_pool, create_descriptor_sets};
use crate::graphics::framebuffers::create_framebuffers;
use crate::graphics::instance::create_instance;
use crate::graphics::logical_device::create_logical_device;
use crate::graphics::models::load_model;
use crate::graphics::physical_device::SuitabilityError;
use crate::graphics::pipeline::{
    create_descriptor_set_layout, create_pipeline, create_render_pass,
};
use crate::graphics::swapchain::{create_swapchain, create_swapchain_image_views};
use crate::graphics::sync_objects::create_sync_objects;
use crate::graphics::textures::{
    create_texture_image, create_texture_image_view, create_texture_sampler,
};
use crate::graphics::uniform_buffer_object::UniformBufferObject;
use crate::graphics::vertex::Vertex;
use crate::AppData;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use vulkanalia::window as vk_window;

#[derive(Copy, Clone, Debug)]
pub(crate) struct QueueFamilyIndices {
    pub(crate) graphics: u32,
    pub(crate) present: u32,
}

impl QueueFamilyIndices {
    pub(crate) unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, properties) in properties.iter().enumerate() {
            if instance.get_physical_device_surface_support_khr(
                physical_device,
                index as u32,
                data.surface,
            )? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError(
                "Missing required queue families."
            )))
        }
    }
}
