use std::collections::HashMap;

use anyhow::{anyhow, Result};
use enigo::{Enigo, MouseControllable};
use winit::event::{
    DeviceId, ElementState, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase,
    VirtualKeyCode,
};
use winit::window::Window;

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
    mouse_delta: (i32, i32),
    last_mouse_delta: (i32, i32),

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
            mouse_delta: (0, 0),
            last_mouse_delta: (0, 0),
            scrolled_up: false,
            scrolled_down: false,
            scroll_delta: 0,
        }
    }

    // Keyboard
    pub(crate) fn get_key_down(&self, key_code: VirtualKeyCode) -> bool {
        self.pressed_current_frame_keyboard.contains(&key_code)
    }

    pub(crate) fn get_key_up(&self, key_code: VirtualKeyCode) -> bool {
        self.released_current_frame_keyboard.contains(&key_code)
    }

    pub(crate) fn get_key(&self, key_code: VirtualKeyCode) -> bool {
        self.currently_pressed_keyboard.contains_key(&key_code)
    }

    // Mouse
    pub(crate) fn get_key_down_mouse(&self, button: MouseButton) -> bool {
        self.pressed_current_frame_mouse.contains(&button)
    }

    pub(crate) fn get_key_up_mouse(&self, button: MouseButton) -> bool {
        self.released_current_frame_mouse.contains(&button)
    }

    pub(crate) fn get_key_mouse(&self, button: MouseButton) -> bool {
        self.currently_pressed_mouse.contains_key(&button)
    }

    pub(crate) fn get_mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    // Wheel
    pub(crate) fn get_scroll(&self, delta: ScrollWheelDelta) -> bool {
        return match delta {
            ScrollWheelDelta::Up => self.scrolled_up,
            ScrollWheelDelta::Down => self.scrolled_down,
        };
    }

    pub(crate) fn get_scroll_delta(self) -> i16 {
        self.scroll_delta
    }

    // Handling
    pub(crate) fn handle_mouse(&mut self, window: &Window, is_cursor_locked: bool) -> Result<()> {
        let window_inner_position = window.inner_position()?;
        let window_inner_size = window.inner_size();

        let window_center_x = window_inner_position.x + window_inner_size.width as i32 / 2;
        let window_center_y = window_inner_position.y + window_inner_size.height as i32 / 2;

        let mouse_position: (i32, i32) = Enigo::mouse_location();

        let x_offset = -(window_center_x - mouse_position.0 - 1);
        let y_offset = -(window_center_y - mouse_position.1 - 1);
        if is_cursor_locked {
            self.mouse_delta = (x_offset, y_offset);
            Enigo.mouse_move_to(window_center_x, window_center_y);
        } else {
            self.mouse_delta = (
                -self.last_mouse_delta.0 + x_offset,
                -self.last_mouse_delta.1 + y_offset,
            );
        }

        self.last_mouse_delta = (x_offset, y_offset);
        Ok(())
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

    pub(crate) fn detect_keyboard(
        &mut self,
        device_id: DeviceId,
        input: KeyboardInput,
        is_synthetic: bool,
        current_frame: u128,
    ) -> Result<()> {
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

    pub(crate) fn detect_mouse(
        &mut self,
        device_id: DeviceId,
        button: MouseButton,
        state: ElementState,
        current_frame: u128,
    ) {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.detected_new_frame();
        }

        match state {
            ElementState::Pressed => {
                if !self.pressed_current_frame_mouse.contains(&button)
                    && !self.currently_pressed_mouse.contains_key(&button)
                {
                    self.pressed_current_frame_mouse.push(button);
                }
                self.currently_pressed_mouse
                    .insert(button, self.currently_pressed_mouse.len());
            }
            ElementState::Released => {
                self.currently_pressed_mouse.remove(&button);
                self.released_current_frame_mouse.push(button);
            }
        }
    }

    pub(crate) fn detect_wheel(
        &mut self,
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
        current_frame: u128,
    ) {
        if current_frame != self.last_frame {
            self.last_frame = current_frame;
            self.detected_new_frame();
        }

        if delta == MouseScrollDelta::LineDelta(0.0, 1.0) {
            self.scrolled_up = true;
            self.scroll_delta += 1;
        } else if delta == MouseScrollDelta::LineDelta(0.0, -1.0) {
            self.scrolled_down = true;
            self.scroll_delta -= 1;
        }
    }
}

pub(crate) enum ScrollWheelDelta {
    Up,
    Down,
}
