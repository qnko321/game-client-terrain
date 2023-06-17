use winit::event::{
    DeviceId, ElementState, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
    VirtualKeyCode,
};
use std::time::Instant;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{KhrSwapchainExtension};
use crate::core::game_object::GameObject;
use anyhow::{anyhow, Result};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use crate::controlls::input_manager::InputManager;
use crate::core::app_data::AppData;
use crate::{FrameData, graphics, HIGH_DELTA_TIME_LIMIT, LOW_DELTA_TIME_LIMIT, MAX_FRAMES_IN_FLIGHT, VALIDATION_ENABLED};
use crate::graphics::command_pool::{create_command_pools, create_text_command_pools};
use crate::graphics::depth_objects::create_depth_objects;
use crate::graphics::framebuffers::create_framebuffers;
use crate::graphics::instance::create_instance;
use crate::graphics::logical_device::create_logical_device;
use crate::graphics::pipeline::{create_descriptor_set_layout, create_pipeline, create_render_pass};
use crate::graphics::swapchain::{create_swapchain, create_swapchain_image_views};
use crate::graphics::text_pipeline::{create_text_descriptor_set_layout, create_text_pipeline, create_text_render_pass};
use crate::terrain::world::World;
use crate::graphics::buffers::{
    create_text_vertex_index_buffers, create_text_vertex_index_buffers_multi,
    create_uniform_buffers,
};
use crate::graphics::command_buffers::{create_command_buffers, create_text_command_buffers};
use crate::graphics::descriptors::{
    create_descriptor_pool, create_descriptor_sets, create_text_descriptor_pool,
    create_text_descriptor_sets,
};
use crate::graphics::font_data::FontData;
use crate::graphics::shared_textures::{create_texture_image_from_path, create_texture_image_view};
use crate::graphics::sync_objects::create_sync_objects;
use crate::graphics::text_object::{TextObject, TextSettings};

use crate::graphics::text_textures::{create_bitmaps, Character};
use crate::graphics::texture_samplers::{
    create_text_texture_sampler, create_world_texture_sampler,
};
use crate::graphics::uniform_buffer_object::UniformBufferObject;
use crate::graphics::vertex::Vertex;
use crate::player::player_data::PlayerData;

use crate::core::collider::Collider;
use crate::core::collision::intersects;

use nalgebra_glm as glm;
use std::collections::HashMap;
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use std::u16;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{DeviceV1_2};
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension};
use vulkanalia::window as vk_window;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};
use crate::terrain::chunk_coord::ChunkCoord;


#[derive(Debug)]
pub(crate) struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    world: World,
    device: Device,
    frame: usize,
    pub(crate) resized: bool,
    start: Instant,
    pub(crate) input_manager: InputManager,

    // State
    pub(crate) is_hovered_by_cursor: bool,

    // Game State
    is_cursor_locked: bool,
    is_playing: bool,

    // Delta Time
    delta_time: f32,
    last_time: Instant,

    is_first_frame: bool,
    pub(crate) frame_count: u128,
}

impl App {
    #[rustfmt::skip]
    pub(crate) unsafe fn create(window: &Window, game_objects: &mut Vec<Box<dyn GameObject>>, new_objects: &mut Vec<usize>) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = AppData::default();

        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, window)?;
        graphics::physical_device::pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&instance, &mut data)?;

        let mut world = World::load(glm::vec3(0.0, 0.0, 0.0));
        world.generate_chunk_voxel_map(&ChunkCoord::zero());
        world.mesh_chunk_sync(&ChunkCoord::zero(), &instance, &device, &mut data);

        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_text_render_pass(&instance, &device, &mut data)?;
        create_text_descriptor_set_layout(&device, &mut data)?;
        create_text_pipeline(&device, &mut data)?;
        create_command_pools(&instance, &device, &mut data)?;
        create_text_command_pools(&instance, &device, &mut data)?;
        create_depth_objects(&instance, &device, &mut data)?;
        create_framebuffers(&device, &mut data)?;

        // 3D
        (data.texture_image, data.texture_image_memory) = create_texture_image_from_path(&instance, &device,&mut data, "resources/blocks.png")?;
        data.texture_image_view = create_texture_image_view(&device, &data.texture_image)?;
        data.texture_sampler = create_world_texture_sampler(&device, &mut data)?;

        // Text
        (data.text_characters, data.text_texture_image, data.text_texture_image_memory) = create_bitmaps(&instance, &device, &mut data)?;

        //data.text_objects.push(TextObject::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", glm::vec2(50.0, 50.0), TextSettings::default()));
        //data.text_objects.push(TextObject::new("Dqdo ti e lezbiika i pravi svirki", glm::vec2(10.0, 540.0), TextSettings::default()));
        data.text_texture_image_view = create_texture_image_view(&device, &data.text_texture_image)?;
        data.text_texture_sampler = create_text_texture_sampler(&device, &mut data)?;
        create_text_vertex_index_buffers(&instance, &device, &mut data)?;

        create_uniform_buffers(&instance, &device, &mut data)?;

        // 3D
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;
        // Text
        create_text_descriptor_pool(&device, &mut data)?;
        create_text_descriptor_sets(&device, &mut data)?;

        create_command_buffers(&device, &mut data)?;
        create_text_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;

        let mut player_data = PlayerData::default();
        player_data.is_grounded = true;
        player_data.velocity = glm::vec3(0.0, 0.0, 0.0);
        player_data.horizontal_angle = 1.57;
        player_data.transform.position = glm::Vec3::new(0.0, 0.0, 50.0);
        player_data.vertical_angle = 3.14;
        player_data.mouse_speed = 1.0;
        player_data.move_speed = 10.0;
        player_data.reach = 10.0;
        player_data.reach_step = 0.01;
        player_data.collider = Collider {
            vertices: vec![
                glm::vec3(0.0, 0.0, 0.0),
                glm::vec3(1.0, 0.0, 0.0),
                glm::vec3(1.0, 1.0, 0.0),
                glm::vec3(0.0, 1.0, 0.0),
                glm::vec3(0.0, 0.0, 1.0),
                glm::vec3(1.0, 0.0, 1.0),
                glm::vec3(1.0, 1.0, 1.0),
                glm::vec3(0.0, 1.0, 1.0),
            ],
        };

        new_objects.push(game_objects.len());
        game_objects.push(Box::new(player_data));

        Ok(Self {
            entry,
            instance,
            data,
            device,
            world: World { chunks: Default::default() },
            input_manager: InputManager::new(),
            frame: 0,
            resized: false,
            start: Instant::now(),
            delta_time: 0.0,
            last_time: Instant::now(),
            is_hovered_by_cursor: false,
            is_cursor_locked: false,
            is_playing: true,
            is_first_frame: true,
            frame_count: 0,
        })
    }

    #[rustfmt::skip]
    pub(crate) unsafe fn render(&mut self, window: &Window, game_objects: &mut Vec<Box<dyn GameObject>>, new_objects: &mut Vec<usize>) -> Result<()> {
        let in_flight_fence = self.data.in_flight_fences[self.frame];

        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        let player = game_objects.get_mut(0).unwrap().as_any_mut().downcast_mut::<PlayerData>().unwrap();

        let frame_data_player = FrameData {
            frame_count: self.frame_count,
            delta_time: self.delta_time,
            input_manager: &self.input_manager,
        };

        // Handle Delta time
        let current_time = Instant::now();
        let delta_time_duration = current_time - self.last_time;
        let mut delta_time_in_seconds = delta_time_duration.as_secs_f64();
        if delta_time_in_seconds < LOW_DELTA_TIME_LIMIT {
            delta_time_in_seconds = LOW_DELTA_TIME_LIMIT;
        } else if delta_time_in_seconds > HIGH_DELTA_TIME_LIMIT {
            delta_time_in_seconds = HIGH_DELTA_TIME_LIMIT;
        }
        self.last_time = current_time;
        self.delta_time = delta_time_in_seconds as f32;

        self.frame_count += 1;

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

        self.update_text_command_buffer(image_index)?;
        self.update_command_buffer(image_index)?;
        let player = game_objects.get_mut(0).unwrap().as_any_mut().downcast_mut::<PlayerData>().unwrap();
        self.update_uniform_buffer(player, image_index)?;

        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.data.command_buffers[image_index], self.data.text_command_buffers[image_index]];
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

        {
            let player = game_objects.get_mut(0).unwrap().as_any_mut().downcast_mut::<PlayerData>().unwrap();

            player.velocity.z -= 9.81 * self.delta_time;

            if player.is_grounded {
                player.velocity.z = 0.0;
            }

            player.transform.position.z += player.velocity.z * self.delta_time;
        }

        let frame_data = FrameData {
            frame_count: self.frame_count,
            delta_time: self.delta_time,
            input_manager: &self.input_manager,
        };

        // Call start on new objects
        new_objects.iter().for_each(|index| {
            let object_index = new_objects.get(*index).unwrap();
            game_objects.get_mut(*object_index).unwrap().start(frame_data.clone());
        });
        new_objects.clear();

        // Call update on all objects
        game_objects.iter_mut().for_each(|obj| {
            obj.update(frame_data.clone());
        });

        if self.is_hovered_by_cursor
            && !self.is_cursor_locked
            && (self.input_manager.get_key_down_mouse(MouseButton::Left) || self.input_manager.get_key_down_mouse(MouseButton::Right))
        {
            self.lock_cursor();
            window.set_cursor_visible(false);
        }

        if self.input_manager.get_key_down(VirtualKeyCode::Escape) {
            self.unlock_cursor();
            window.set_cursor_visible(true);
        }

        // Input
        self.input_manager.handle_mouse(window, self.is_cursor_locked).expect("Couldn't handle mouse delta input");

        if self.input_manager.get_key_down(VirtualKeyCode::F11) {
            self.toggle_fullscreen(window);
        }
        if self.input_manager.get_key_down(VirtualKeyCode::Space) {
            game_objects.get_mut(0).unwrap().as_any_mut().downcast_mut::<PlayerData>().unwrap().is_grounded = !game_objects.get(0).unwrap().as_any().downcast_ref::<PlayerData>().unwrap().is_grounded;
        }

        let player_pos = game_objects.get(0).unwrap().as_any().downcast_ref::<PlayerData>().unwrap().transform.position;

        //TODO: update view distance
        /*// Terrain
        self.world.update_view_distance(
            player_pos.x as i32,
            player_pos.y as i32,
            player_pos.z as i32,
            &self.instance,
            &self.device,
            &mut self.data,
        )?;*/

        self.input_manager.detected_new_frame();

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    #[rustfmt::skip]
    fn toggle_fullscreen(&mut self, window: &Window) {
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

    #[rustfmt::skip]
    unsafe fn update_secondary_command_buffer(&mut self, image_index: usize, model_index: usize) -> Result<vk::CommandBuffer> {
        //TODO: FIX
        let command_buffers = &mut self.data.secondary_command_buffers[image_index];

        while model_index >= command_buffers.len() {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(self.data.command_pools[image_index])
                .level(vk::CommandBufferLevel::SECONDARY)
                .command_buffer_count(1);

            let command_buffer = self.device.allocate_command_buffers(&allocate_info)?[0];
            command_buffers.push(command_buffer);
        }

        let command_buffer = command_buffers[model_index];

        // TODO: Properly handle errors
        let chunk = self.world.get_chunk_by_index(model_index).unwrap();

        if !chunk.should_draw() {
            return Err(anyhow!("Don't draw chunk"));
        }

        let binding = chunk.get_model_matrix();
        let (_, model_bytes, _) = binding.as_slice().align_to::<u8>();

        let opacity: f32 = 1.0;//(model_index + 1) as f32 * 0.25;
        let opacity_bytes = &opacity.to_ne_bytes()[..];

        let inheritance_info = vk::CommandBufferInheritanceInfo::builder()
            .render_pass(self.data.render_pass)
            .subpass(0)
            .framebuffer(self.data.framebuffers[image_index]);

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE)
            .inheritance_info(&inheritance_info);

        self.device.begin_command_buffer(command_buffer, &info)?;

        self.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.pipeline,
        );
        self.device.cmd_bind_vertex_buffers(command_buffer, 0, &[chunk.get_vertex_buffer()], &[0]);
        self.device.cmd_bind_index_buffer(command_buffer, chunk.get_index_buffer(), 0, vk::IndexType::UINT32);
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
            opacity_bytes,
        );
        self.device.cmd_draw_indexed(command_buffer, chunk.get_mesh().indices.len() as u32 + chunk.new_indices_count, 1, 0, 0, 0);

        self.device.end_command_buffer(command_buffer)?;
        Ok(command_buffer)
    }

    #[rustfmt::skip]
    unsafe fn update_text_secondary_command_buffer(&mut self, image_index: usize, object_index: usize) -> Result<vk::CommandBuffer> {
        let command_buffers = &mut self.data.text_secondary_command_buffers[image_index];

        while object_index >= command_buffers.len() {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(self.data.text_command_pools[image_index])
                .level(vk::CommandBufferLevel::SECONDARY)
                .command_buffer_count(1);

            let command_buffer = self.device.allocate_command_buffers(&allocate_info)?[0];
            command_buffers.push(command_buffer);
        }

        let command_buffer = command_buffers[object_index];

        let text_object = self.data.text_objects.get(object_index);

        let inheritance_info = vk::CommandBufferInheritanceInfo::builder()
            .render_pass(self.data.text_render_pass)
            .subpass(0)
            .framebuffer(self.data.framebuffers[image_index]);

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE)
            .inheritance_info(&inheritance_info);

        self.device.begin_command_buffer(command_buffer, &info)?;

        self.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.text_pipeline,
        );
        self.device.cmd_bind_vertex_buffers(command_buffer, 0, &[self.data.text_vertex_buffer], &[0]);
        self.device.cmd_bind_index_buffer(command_buffer, self.data.text_index_buffer, 0, vk::IndexType::UINT32);
        self.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            self.data.text_pipeline_layout,
            0,
            &[self.data.text_descriptor_sets[image_index]],
            &[],
        );

        self.device.cmd_draw_indexed(command_buffer, self.data.text_index_buffer_length as u32, 1, 0, 0, 0);

        self.device.end_command_buffer(command_buffer)?;
        Ok(command_buffer)
    }

    #[rustfmt::skip]
    unsafe fn update_command_buffer(&mut self, image_index: usize) -> Result<()> {
        let command_pool = self.data.command_pools[image_index];
        self.device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;

        let command_buffer = self.data.command_buffers[image_index];

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        self.device.begin_command_buffer(command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(self.data.swapchain_extent);

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        };

        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        };

        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.data.render_pass)
            .framebuffer(self.data.framebuffers[image_index])
            .render_area(render_area)
            .clear_values(clear_values);

        self.device.cmd_begin_render_pass(command_buffer, &info, vk::SubpassContents::SECONDARY_COMMAND_BUFFERS);

        let mut secondary_command_buffers = Vec::<vk::CommandBuffer>::new();
        //TOOD: FIX
        /*for chunk_index in 0..self.world.chunks_len() {
            match self.update_secondary_command_buffer(image_index, chunk_index) {
                Ok(buffer) => secondary_command_buffers.push(buffer),
                Err(_) => {},
            }
        }*/

        self.device.cmd_execute_commands(command_buffer, &secondary_command_buffers);

        self.device.cmd_end_render_pass(command_buffer);

        self.device.end_command_buffer(command_buffer)?;

        Ok(())
    }

    #[rustfmt::skip]
    unsafe fn update_text_command_buffer(&mut self, image_index: usize) -> Result<()> {
        let command_pool = self.data.text_command_pools[image_index];
        self.device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty())?;

        let command_buffer = self.data.text_command_buffers[image_index];

        let info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        self.device.begin_command_buffer(command_buffer, &info)?;

        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(self.data.swapchain_extent);

        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [1.0, 0.0, 0.0, 0.2],
            },
        };

        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        };

        let clear_values = &[color_clear_value, depth_clear_value];
        let info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.data.text_render_pass)
            .framebuffer(self.data.framebuffers[image_index])
            .render_area(render_area)
            .clear_values(clear_values);

        self.device.cmd_begin_render_pass(command_buffer, &info, vk::SubpassContents::SECONDARY_COMMAND_BUFFERS);

        let mut secondary_command_buffers = Vec::<vk::CommandBuffer>::new();
        for text_index in 0..self.data.text_objects.len() {
            match self.update_text_secondary_command_buffer(image_index, text_index) {
                Ok(buffer) => secondary_command_buffers.push(buffer),
                Err(error) => {println!("Error updating text secondary command buffer: {}", error)},
            }
        }

        self.device.cmd_execute_commands(command_buffer, &secondary_command_buffers);

        self.device.cmd_end_render_pass(command_buffer);

        self.device.end_command_buffer(command_buffer)?;

        Ok(())
    }

    #[rustfmt::skip]
    unsafe fn update_uniform_buffer(&self, player: &mut PlayerData, image_index: usize) -> Result<()> {
        // MVP

        let look_direction = glm::vec3(
            (player.vertical_angle.cos() * player.horizontal_angle.sin()) as f32,
            (player.vertical_angle.cos() * player.horizontal_angle.cos()) as f32,
            player.vertical_angle.sin() as f32,
        );

        let right = glm::vec3(
            (player.horizontal_angle - 3.14 / 2.0).sin() as f32,
            (player.horizontal_angle - 3.14 / 2.0).cos() as f32,
            0.0,
        );

        let up = glm::cross(&right, &look_direction);

        let center = glm::vec3(
            player.transform.position.x + look_direction.x as f32,
            player.transform.position.y + look_direction.y as f32,
            player.transform.position.z + look_direction.z as f32,
        );

        let view = glm::look_at(&player.transform.position, &center, &up);

        let mut proj = glm::perspective_rh_zo(
            self.data.swapchain_extent.width as f32 / self.data.swapchain_extent.height as f32,
            glm::radians(&glm::vec1(90.0))[0],
            0.1,
            100.0,
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

    #[rustfmt::skip]
    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain();
        create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;

        // 3D pipeline
        create_render_pass(&self.instance, &self.device, &mut self.data)?;
        create_pipeline(&self.device, &mut self.data)?;

        // Text Pipeline
        create_text_render_pass(&self.instance, &self.device, &mut self.data)?;
        create_text_pipeline(&self.device, &mut self.data)?;

        create_depth_objects(&self.instance, &self.device, &mut self.data)?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
        // 3D
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        // Text
        create_text_descriptor_pool(&self.device, &mut self.data)?;
        create_text_descriptor_sets(&self.device, &mut self.data)?;

        create_command_buffers(&self.device, &mut self.data)?;
        create_text_command_buffers(&self.device, &mut self.data)?;
        self.data.images_in_flight.resize(self.data.swapchain_images.len(), vk::Fence::null());
        Ok(())
    }

    #[rustfmt::skip]
    pub(crate) unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();

        self.destroy_swapchain();
        self.data.command_pools.iter().for_each(|p| self.device.destroy_command_pool(*p, None));
        self.data.text_command_pools.iter().for_each(|p| self.device.destroy_command_pool(*p, None));
        self.data.in_flight_fences.iter().for_each(|f| self.device.destroy_fence(*f, None));
        self.data.render_finished_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.image_available_semaphores.iter().for_each(|s| self.device.destroy_semaphore(*s, None));
        self.world.destroy(&self.device);
        self.device.free_memory(self.data.text_index_buffer_memory, None);
        self.device.destroy_buffer(self.data.text_index_buffer, None);
        self.device.free_memory(self.data.text_vertex_buffer_memory, None);
        self.device.destroy_buffer(self.data.text_vertex_buffer, None);
        self.device.free_memory(self.data.index_buffer_memory, None);
        self.device.destroy_buffer(self.data.index_buffer, None);
        self.device.free_memory(self.data.vertex_buffer_memory, None);
        self.device.destroy_buffer(self.data.vertex_buffer, None);
        self.device.destroy_sampler(self.data.texture_sampler, None);
        self.device.destroy_image_view(self.data.texture_image_view, None);
        self.device.free_memory(self.data.texture_image_memory, None);
        self.device.destroy_image(self.data.texture_image, None);
        self.device.destroy_sampler(self.data.text_texture_sampler, None);
        self.device.destroy_image_view(self.data.text_texture_image_view, None);
        self.device.free_memory(self.data.text_texture_image_memory, None);
        self.device.destroy_image(self.data.text_texture_image, None);
        self.device.destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);
        self.device.destroy_descriptor_set_layout(self.data.text_descriptor_set_layout, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
        self.instance.destroy_instance(None);
    }

    #[rustfmt::skip]
    unsafe fn destroy_swapchain(&mut self) {
        self.device.destroy_image_view(self.data.depth_image_view, None);
        self.device.free_memory(self.data.depth_image_memory, None);
        self.device.destroy_image(self.data.depth_image, None);
        self.device.destroy_descriptor_pool(self.data.descriptor_pool, None);
        self.device.destroy_descriptor_pool(self.data.text_descriptor_pool, None);
        self.data.uniform_buffers_memory.iter().for_each(|m| self.device.free_memory(*m, None));
        self.data.uniform_buffers.iter().for_each(|b| self.device.destroy_buffer(*b, None));
        self.data.framebuffers.iter().for_each(|f| self.device.destroy_framebuffer(*f, None));
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);
        self.device.destroy_pipeline(self.data.text_pipeline, None);
        self.device.destroy_pipeline_layout(self.data.text_pipeline_layout, None);
        self.device.destroy_render_pass(self.data.text_render_pass, None);
        self.data.swapchain_image_views.iter().for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }

    fn lock_cursor(&mut self) {
        self.is_cursor_locked = true;
        self.is_first_frame = true;
    }

    pub(crate) fn unlock_cursor(&mut self) {
        self.is_cursor_locked = false;
        self.is_first_frame = true;
    }
}