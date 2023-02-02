use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aws_sdk_dynamodb::model::KeyType::Hash;
use winit::event::{DeviceId, ElementState, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode};
use crate::graphics::shared_images::copy_buffer_to_image;

#[derive(Clone, Debug)]
pub(crate) struct InputManager {
    last_frame: u128,

    // Keyboard
    currently_pressed_keyboard: HashMap<VirtualKeyCode, usize>,
    pressed_current_frame_keyboard: Vec<VirtualKeyCode>,
    released_current_frame_keyboard: Vec<VirtualKeyCode>,

    // Mouse
    currently_pressed_mouse: HashMap<MouseButton, usize>,
    pressed_current_frame_mouse: Vec<MouseButton>,
    released_current_frame_mouse: Vec<MouseButton>,

    // Wheel
    scrolled_up: bool,
    scrolled_down: bool,
    scroll_delta: i16,
}

impl InputManager {
    pub(crate) fn new() -> Self {
        Self {
            last_frame: 1,
            currently_pressed_keyboard: HashMap::new(),
            pressed_current_frame_keyboard: vec![],
            released_current_frame_keyboard: vec![],
            currently_pressed_mouse: HashMap::new(),
            pressed_current_frame_mouse: vec![],
            released_current_frame_mouse: vec![],
            scrolled_up: false,
            scrolled_down: false,
            scroll_delta: 0,
        }
    }

    pub(crate) fn detect_keyboard(&mut self, device_id: DeviceId, input: KeyboardInput, is_synthetic: bool, current_frame: u128, ) -> Result<()> {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.detected_new_frame();
        }

        let is_valid = input.virtual_keycode.is_some();

        if !is_valid {
            return Err(anyhow!("Couldn't read virtual keycode!"));
        }

        let key_code = input.virtual_keycode.unwrap();

        match input.state {
            ElementState::Pressed => {
                if !self.pressed_current_frame_keyboard.contains(&key_code)
                    && !self.currently_pressed_keyboard.contains_key(&key_code)
                {
                    self.pressed_current_frame_keyboard.push(key_code);
                }
                self.currently_pressed_keyboard
                    .insert(key_code, self.currently_pressed_keyboard.len());
            }
            ElementState::Released => {
                self.currently_pressed_keyboard.remove(&key_code);
                self.released_current_frame_keyboard.push(key_code);
            }
        }

        Ok(())
    }

    pub(crate) fn detect_mouse(&mut self, device_id: DeviceId, button: MouseButton, state: ElementState, current_frame: u128) {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.detected_new_frame();
        }

        match state {
            ElementState::Pressed => {
                if !self.pressed_current_frame_mouse.contains(&button) && !self.currently_pressed_mouse.contains_key(&button) {
                    self.pressed_current_frame_mouse.push(button);
                }
                self.currently_pressed_mouse.insert(button, self.currently_pressed_mouse.len());
            }
            ElementState::Released => {
                self.currently_pressed_mouse.remove(&button);
                self.released_current_frame_mouse.push(button);
            }
        }
    }

    pub(crate) fn detect_wheel(&mut self, device_id: DeviceId, delta: MouseScrollDelta, phase: TouchPhase, current_frame: u128) {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.detected_new_frame();
        }

        match delta {
            MouseScrollDelta::LineDelta(0_f32, 1_f32) => {
                self.scrolled_up = true;
                self.scroll_delta += 1;
            }
            MouseScrollDelta::LineDelta(0_f32, -1_f32) => {
                self.scrolled_down = true;
                self.scroll_delta -= 1;
            }
            _ => {},
        }
    }

    pub(crate) fn detected_new_frame(&mut self) {
        self.pressed_current_frame_keyboard.clear();
        self.released_current_frame_keyboard.clear();
        self.pressed_current_frame_mouse.clear();
        self.released_current_frame_mouse.clear();
        self.scrolled_up = false;
        self.scrolled_down = false;
        self.scroll_delta = 0;
    }

    // Keyboard
    pub(crate) fn get_key_down(&mut self, key_code: VirtualKeyCode) -> bool {
        self.pressed_current_frame_keyboard.contains(&key_code)
    }

    pub(crate) fn get_key_up(&mut self, key_code: VirtualKeyCode) -> bool {
        self.released_current_frame_keyboard.contains(&key_code)
    }

    pub(crate) fn get_key(&mut self, key_code: VirtualKeyCode) -> bool {
        self.currently_pressed_keyboard.contains_key(&key_code)
    }

    // Mouse
    pub(crate) fn get_key_down_mouse(&mut self, button: MouseButton) -> bool {
        self.pressed_current_frame_mouse.contains(&button)
    }

    pub(crate) fn get_key_up_mouse(&mut self, button: MouseButton) -> bool {
        self.released_current_frame_mouse.contains(&button)
    }

    pub(crate) fn get_key_mouse(&mut self, button: MouseButton) -> bool {
        self.currently_pressed_mouse.contains_key(&button)
    }

    // Wheel
    pub(crate) fn get_scroll(&mut self, delta: ScrollWheelDelta) -> bool {
        return match delta {
            ScrollWheelDelta::Up => self.scrolled_up,
            ScrollWheelDelta::Down => self.scrolled_down,
        };
    }

    pub(crate) fn get_scroll_delta(&mut self) -> i16 {
        self.scroll_delta
    }
}

pub(crate) enum ScrollWheelDelta {
    Up,
    Down
}