use std::collections::HashMap;
use log::{error, info};
use winit::event::{DeviceId, ElementState, KeyboardInput, VirtualKeyCode};
use anyhow::{anyhow, Result};
use crate::App;

#[derive(Clone, Debug)]
pub(crate) struct InputManager {
    last_frame: u128,
    currently_pressed: HashMap<VirtualKeyCode, usize>,
    pressed_current_frame: Vec<VirtualKeyCode>,
    released_current_frame: Vec<VirtualKeyCode>,
}

impl InputManager {
    pub(crate) fn new() -> Self {
        Self {
            last_frame: 1,
            currently_pressed: HashMap::new(),
            pressed_current_frame: vec![],
            released_current_frame: vec![],
        }
    }

    pub(crate) fn detect_change(&mut self, device_id: DeviceId, input: KeyboardInput, is_synthetic: bool, current_frame: u128) -> Result<()> {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.pressed_current_frame.clear();
            self.released_current_frame.clear();
        }

        let is_valid = input.virtual_keycode.is_some();

        if !is_valid {
            return Err(anyhow!("Couldn't read virtual keycode!"));
        }

        let key_code = input.virtual_keycode.unwrap();

        match input.state {
            ElementState::Pressed => {
                if !self.pressed_current_frame.contains(&key_code) && !self.currently_pressed.contains_key(&key_code) {
                    self.pressed_current_frame.push(key_code);
                }
                self.currently_pressed.insert(key_code, self.currently_pressed.len());
            }
            ElementState::Released => {
                self.currently_pressed.remove(&key_code);
                self.released_current_frame.push(key_code);
            }
        }
        Ok(())
    }

    pub(crate) fn get_key_down(&mut self, key_code: VirtualKeyCode) -> bool {
        self.pressed_current_frame.contains(&key_code)
    }

    pub(crate) fn get_key_up(&mut self, key_code: VirtualKeyCode) -> bool {
        self.released_current_frame.contains(&key_code)
    }

    pub(crate) fn get_key(&mut self, key_code: VirtualKeyCode) -> bool {
        self.currently_pressed.contains_key(&key_code)
    }
}