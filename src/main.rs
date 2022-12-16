#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
//
mod graphics;
mod player;
mod controlls;
mod core;
//
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::{copy_nonoverlapping as memcpy, null};
use std::time::Instant;
//
use anyhow::{anyhow, Result};
use log::*;
use nalgebra_glm as glm;
use nalgebra_glm::{clamp, cos, is_null, sin, TVec3};
use thiserror::Error;
//
use winit::dpi::{LogicalPosition, LogicalSize, Position};
use winit::event::{ElementState, Event, ScanCode, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorIcon, Fullscreen, Window, WindowBuilder};
//
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
use enigo::*;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtension;
use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::vk::KhrSwapchainExtension;
use vulkanalia::window as vk_window;
use winit::monitor::VideoMode;
use crate::controlls::input_manager;
use crate::controlls::input_manager::InputManager;
use crate::core::transform::Transform;
use crate::core::trigonometry_shared::FastConversions;
use crate::player::player_data::PlayerData;
use crate::core::game_object;
use crate::core::game_object::GameObject;
use crate::core::model::Model;

//Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
//The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

//The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

//The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Kub4e")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    let mut minimized = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Render a frame if our Vulkan app is not being destroyed.
            Event::MainEventsCleared if !destroying && !minimized => unsafe { app.render(&window) }.unwrap(),
            // Mark the window as having been resized.
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    app.resized = true;
                }
            }
            // Destroy our Vulkan app.
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            Event::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                app.set_focused(focused);
                window.set_cursor_visible(!focused);
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput {device_id, input, is_synthetic}, .. } => {
                if app.input_manager.detect_change(device_id, input, is_synthetic, app.frame_count).is_err() {
                    app.set_focused(false);
                    window.set_cursor_visible(true);
                }
            },
            _ => {}
        }
    });
}

// Our Vulkan app.
#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
    frame: usize,
    resized: bool,
    start: Instant,
    is_focused: bool,
    input_manager: InputManager,
    // Delta Time
    delta_time: f32,
    last_frame_time: Instant,
    is_first_frame: bool,
    frame_count: u128,
    player_data: PlayerData,
}
//
impl App {
    // Creates our Vulkan app.
    unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, window)?;
        graphics::physical_device::pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_command_pools(&instance, &device, &mut data)?;
        create_depth_objects(&instance, &device, &mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_texture_image(&instance, &device, &mut data)?;
        create_texture_image_view(&device, &mut data)?;
        create_texture_sampler(&device, &mut data)?;
        load_model(&mut data)?;
        create_vertex_buffer(&instance, &device, &mut data)?;
        create_index_buffer(&instance, &device, &mut data)?;
        create_uniform_buffers(&instance, &device, &mut data)?;
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;

        let transform = Transform{
            position: glm::Vec3::new(-2.0, 0.0, 0.0),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
        };

        create_command_buffers(&device, &mut data, &transform.get_model_matrix())?; //TODO: FIx
        create_sync_objects(&device, &mut data)?;

        let mut player_data = PlayerData::default();
        player_data.horizontal_angle = 1.57;
        player_data.transform.position = glm::Vec3::new(4.0, 0.0, 0.0);
        player_data.vertical_angle = 3.14;
        player_data.mouse_speed = 2.0;
        player_data.move_speed = 1.0;

        Ok(Self {
            entry,
            instance,
            data,
            device,
            input_manager: InputManager::new(),
            frame: 0,
            resized: false,
            start: Instant::now(),
            last_frame_time: Instant::now(),
            delta_time: 0.0,
            is_focused: true,
            is_first_frame: true,
            frame_count: 0,
            player_data
        })
    }

    // Renders a frame for our Vulkan app.
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        self.frame_count += 1;
        let current_frame_time = Instant::now();
        self.delta_time = (current_frame_time - self.last_frame_time).as_secs_f32();

        let result = self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::MAX,
            self.data.image_available_semaphores[self.frame],
            vk::Fence::null(),
        );

        let image_index = match result {
            Ok((image_index, _)) => image_index as usize,
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e) => return Err(anyhow!(e)),
        };

        let image_in_flight = self.data.images_in_flight[image_index];
        if !image_in_flight.is_null() {
            self.device
                .wait_for_fences(&[image_in_flight], true, u64::MAX)?;
        }

        self.data.images_in_flight[image_index] = in_flight_fence;

        self.update_command_buffer(image_index)?;
        self.update_uniform_buffer(image_index)?;

        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index]];
        let signal_semaphores = &[self.data.render_finished_semaphores[self.frame]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        self.device.reset_fences(&[in_flight_fence])?;

        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let result = self
            .device
            .queue_present_khr(self.data.present_queue, &present_info);
        let changed = result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
            || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);
        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain(window)?;
        } else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        if self.is_focused {
            if !self.is_first_frame {
                let window_inner = window.inner_position()?;
                let mouse_location: (i32, i32) = Enigo::mouse_location();
                let x_offset = window_inner.x + self.data.swapchain_extent.width as i32 / 2_i32
                    - mouse_location.0
                    - 1;
                let y_offset = window_inner.y + self.data.swapchain_extent.height as i32 / 2_i32
                    - mouse_location.1
                    - 1;
                let x_offset = -x_offset;
                let y_offset = -y_offset;

                self.player_data.horizontal_angle += self.delta_time
                    * self.player_data.mouse_speed
                    * x_offset as f32;
                self.player_data.vertical_angle += (current_frame_time - self.last_frame_time).as_secs_f32()
                    * self.player_data.mouse_speed
                    * y_offset as f32;

                self.player_data.vertical_angle = glm::clamp_scalar(self.player_data.vertical_angle, 0.0, 6.28);
            } else {
                self.is_first_frame = false;
            }

            let window_inner = window.inner_position()?;
            let center_of_window_x =
                window_inner.x + self.data.swapchain_extent.width as i32 / 2_i32;
            let center_of_window_y =
                window_inner.y + self.data.swapchain_extent.height as i32 / 2_i32;
            Enigo.mouse_move_to(center_of_window_x, center_of_window_y);
        }

        if self.input_manager.get_key(VirtualKeyCode::W) {
            let mut forward = self.player_data.forward();
            forward.z = 0.0;
            self.player_data.walk(forward, self.delta_time);
        }
        if self.input_manager.get_key(VirtualKeyCode::S) {
            let mut backward = -self.player_data.forward();
            backward.z = 0.0;
            self.player_data.walk(backward, self.delta_time);
        }
        if self.input_manager.get_key(VirtualKeyCode::D) {
            let mut right = self.player_data.right();
            self.player_data.walk(right, self.delta_time);
        }
        if self.input_manager.get_key(VirtualKeyCode::A) {
            let mut left = -self.player_data.right();
            self.player_data.walk(left, self.delta_time);
        }

        if self.input_manager.get_key_down(VirtualKeyCode::F11) {
            if window.fullscreen().is_some() {
                window.set_fullscreen(None);
            } else {
                window.current_monitor().map(|monitor| {
                    monitor.video_modes().next().map(|video_mode| {
                        if cfg!(any(target_os = "macos", unix)) {
                            window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                        } else {
                            window.set_fullscreen(Some(Fullscreen::Exclusive(video_mode)));
                        }
                    })
                });
            }
            self.resized = true;
            self.is_first_frame = true;
        }

        if self.input_manager.get_key_down(VirtualKeyCode::Space) {
            info!("Created a model");
        }

        self.input_manager.detect_new_frame();

        self.last_frame_time = current_frame_time;

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    unsafe fn update_command_buffer(&mut self, image_index: usize) -> Result<()> {
        let command_pool = self.data.command_pools[image_index];
        self.device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;

        let command_buffer = self.data.command_buffers[image_index];
        self.data.command_buffers[image_index] = command_buffer;

        let transform = Transform{
            position: glm::Vec3::new(-2.0, 0.0, 0.0),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
        };
//
        let model = transform.get_model_matrix();
//
        let (_, model_bytes, _) = model.as_slice().align_to::<u8>();
//
        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
//
        self.device.begin_command_buffer(command_buffer, &info)?;
//
        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(self.data.swapchain_extent);
//
        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };
//
        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        };
//
        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.data.render_pass)
            .framebuffer(self.data.framebuffers[image_index])
            .render_area(render_area)
            .clear_values(clear_values);
//
        self.device.cmd_begin_render_pass(command_buffer, &info, vk::SubpassContents::INLINE);
        self.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.pipeline,
        );
        self.device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.data.vertex_buffer], &[0]);
        self.device.cmd_bind_index_buffer(command_buffer, self.data.index_buffer, 0, vk::IndexType::UINT32);
        self.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.pipeline_layout,
            0,
            &[self.data.descriptor_sets[image_index]],
            &[],
        );
        self.device.cmd_push_constants(
            command_buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            model_bytes,
        );
        self.device.cmd_push_constants(
            command_buffer,
            self.data.pipeline_layout,
            vk::ShaderStageFlags::FRAGMENT,
            64,
            &0.25f32.to_ne_bytes()[..],
        );
        self.device.cmd_draw_indexed(command_buffer, self.data.indices.len() as u32, 1, 0, 0, 0);
        self.device.cmd_end_render_pass(command_buffer);

        self.device.end_command_buffer(command_buffer)?;

        Ok(())
    }

    //Updates the uniform buffer object for our Vulkan app.
    unsafe fn update_uniform_buffer(&self, image_index: usize) -> Result<()> {
        // MVP

        let time = self.start.elapsed().as_secs_f32();

        let look_direction = glm::vec3(
            (self.player_data.vertical_angle.cos() * self.player_data.horizontal_angle.sin()) as f32,
            (self.player_data.vertical_angle.cos() * self.player_data.horizontal_angle.cos()) as f32,
            self.player_data.vertical_angle.sin() as f32,
        );

        let right = glm::vec3(
            (self.player_data.horizontal_angle - 3.14 / 2.0).sin() as f32,
            (self.player_data.horizontal_angle - 3.14 / 2.0).cos() as f32,
            0.0,
        );

        let up = glm::cross(&right, &look_direction);

        let center = glm::vec3(
            self.player_data.transform.position.x + look_direction.x as f32,
            self.player_data.transform.position.y + look_direction.y as f32,
            self.player_data.transform.position.z + look_direction.z as f32,
        );

        let view = glm::look_at(&self.player_data.transform.position, &center, &up);

        let mut proj = glm::perspective_rh_zo(
            self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
            glm::radians(&glm::vec1(45.0))[0],
            0.1,
            10.0,
        );

        proj[(1, 1)] *= -1.0;

        let ubo = UniformBufferObject { view, proj };

        // Copy

        let memory = self.device.map_memory(
            self.data.uniform_buffers_memory[image_index],
            0,
            size_of::<UniformBufferObject>() as u64,
            vk::MemoryMapFlags::empty(),
        )?;

        memcpy(&ubo, memory.cast(), 1);

        self.device
            .unmap_memory(self.data.uniform_buffers_memory[image_index]);

        Ok(())
    }

    /// Recreates the swapchain for our Vulkan app.
    #[rustfmt::skip]
    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain();
        create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;
        create_render_pass(&self.instance, &self.device, &mut self.data)?;
        create_pipeline(&self.device, &mut self.data)?;
        create_depth_objects(&self.instance, &self.device, &mut self.data)?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        let transform = Transform{
            position: glm::Vec3::new(-2.0, 0.0, 0.0),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
        };
        create_command_buffers(&self.device, &mut self.data, &transform.get_model_matrix())?;
        self.data.images_in_flight.resize(self.data.swapchain_images.len(), vk::Fence::null());
        Ok(())
    }

        //Destroys our Vulkan app.
    #[rustfmt::skip]
    unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();
        self.data.command_pools.iter().for_each(|p| self.device.destroy_command_pool(*p, None));
        self.data.in_flight_fences.iter().for_each(|f| self.device.destroy_fence(*f, None));
        self.data.render_finished_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.device.free_memory(self.data.index_buffer_memory, None);
        self.device.destroy_buffer(self.data.index_buffer, None);
        self.device.free_memory(self.data.vertex_buffer_memory, None);
        self.device.destroy_buffer(self.data.vertex_buffer, None);
        self.device.destroy_sampler(self.data.texture_sampler, None);
        self.device.destroy_image_view(self.data.texture_image_view, None);
        self.device.free_memory(self.data.texture_image_memory, None);
        self.device.destroy_image(self.data.texture_image, None);
        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }

    //Destroys the parts of our Vulkan app related to the swapchain.
    #[rustfmt::skip]
    unsafe fn destroy_swapchain(&mut self) {
        self.device.destroy_image_view(self.data.depth_image_view, None);
        self.device.free_memory(self.data.depth_image_memory, None);
        self.device.destroy_image(self.data.depth_image, None);
        self.device.destroy_descriptor_pool(self.data.descriptor_pool, None);
        self.data.uniform_buffers_memory.iter().for_each(|m| self.device.free_memory(*m, None));
        self.data.uniform_buffers.iter().for_each(|b| self.device.destroy_buffer(*b, None));
        self.data.framebuffers.iter().for_each(|f| self.device.destroy_framebuffer(*f, None));
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);
        self.data.swapchain_image_views.iter().for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }
//
    fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        self.is_first_frame = true;
    }
}
//
// The Vulkan handles and associated properties used by our Vulkan app.
#[derive(Clone, Debug, Default)]
struct AppData {
    // Debug
    messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    surface: vk::SurfaceKHR,
    // Physical Device / Logical Device
    physical_device: vk::PhysicalDevice,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    // Swapchain
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,
    // Pipeline
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    // Framebuffers
    framebuffers: Vec<vk::Framebuffer>,
    // Command Pool
    command_pool: vk::CommandPool,
    // Texture
    texture_image: vk::Image,
    texture_image_memory: vk::DeviceMemory,
    texture_image_view: vk::ImageView,
    texture_sampler: vk::Sampler,
    // 3D Objects
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    // Buffers
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    uniform_buffers: Vec<vk::Buffer>,
    uniform_buffers_memory: Vec<vk::DeviceMemory>,
    // Descriptors
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    // Commands
    command_pools: Vec<vk::CommandPool>,
    command_buffers: Vec<vk::CommandBuffer>,
    // Sync Objects
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    images_in_flight: Vec<vk::Fence>,
    // Depth
    depth_image: vk::Image,
    depth_image_memory: vk::DeviceMemory,
    depth_image_view: vk::ImageView,
}