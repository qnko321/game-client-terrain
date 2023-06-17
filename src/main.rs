//#![windows_subsystem = "windows"]

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod controlls;
mod core;
mod graphics;
mod player;
mod terrain;

use crate::controlls::input_manager::InputManager;
use crate::core::collider::Collider;
use crate::core::collision::intersects;
use crate::core::game_object::GameObject;

use anyhow::{anyhow, Result};
use nalgebra_glm as glm;
use std::collections::HashMap;
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use std::time::Instant;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{DeviceV1_2, KhrSwapchainExtension};
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension};
use vulkanalia::window as vk_window;
use winit::dpi::LogicalSize;
use winit::event::{Event, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};
use crate::core::app::App;
use crate::terrain::chunk_coord::ChunkCoord;
use crate::terrain::world::World;

//Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = false; //
                                        //The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

//The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

//The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;

const LOW_DELTA_TIME_LIMIT: f64 = 0.0005;

const HIGH_DELTA_TIME_LIMIT: f64 = 0.4;

#[derive(Clone)]
pub(crate) struct FrameData<'a> {
    pub(crate) delta_time: f32,
    pub(crate) frame_count: u128,
    pub(crate) input_manager: &'a InputManager,
}

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Kub4e")
        .with_inner_size(LogicalSize::new(1024, 576))
        .build(&event_loop)?;

    let mut game_objects: Vec<Box<dyn GameObject>> = vec![];
    let mut new_objects: Vec<usize> = vec![];

    let mut app = unsafe { App::create(&window, &mut game_objects, &mut new_objects)? };
    let mut destroying = false;
    let mut minimized = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::MainEventsCleared if !destroying && !minimized => unsafe { app.render(&window, &mut game_objects, &mut new_objects) }.unwrap(),
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                if size.width == 0 || size.height == 0 {
                    minimized = true;
                } else {
                    minimized = false;
                    app.resized = true;
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput {device_id, input, is_synthetic}, .. } => {
                if app.input_manager.detect_keyboard(device_id, input, is_synthetic, app.frame_count).is_err() {
                    app.unlock_cursor();
                    window.set_cursor_visible(true);
                }
            },
            Event::WindowEvent { event: WindowEvent::MouseInput {state, button, device_id, ..}, ..} => {
                app.input_manager.detect_mouse(device_id, button, state, app.frame_count);
            },
            Event::WindowEvent { event: WindowEvent::MouseWheel {device_id, delta, phase, ..}, ..} => {
                app.input_manager.detect_wheel(device_id, delta, phase, app.frame_count);
            },
            Event::WindowEvent { event: WindowEvent::CursorLeft {device_id}, ..} => {
                app.is_hovered_by_cursor = false;
            }
            Event::WindowEvent { event: WindowEvent::CursorEntered {device_id}, ..} => {
                app.is_hovered_by_cursor = true;
            }
            _ => {}
        }
    });
}
