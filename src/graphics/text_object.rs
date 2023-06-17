use nalgebra_glm as glm;
#[derive(Clone, Debug)]
pub(crate) struct TextObject {
    content: &'static str,
    position: glm::Vec2,
    settings: TextSettings,
    chars: Vec<u32>,
}

impl TextObject {
    pub(crate) fn new(content: &'static str, position: glm::Vec2, settings: TextSettings) -> Self {
        let mut chars = vec![];

        for char in content.chars() {
            chars.push(char as u32);
        }

        Self {
            content,
            position,
            settings,
            chars,
        }
    }

    pub(crate) fn get_content(&self) -> &'static str {
        self.content
    }

    pub(crate) fn get_position(&self) -> glm::Vec2 {
        self.position
    }

    pub(crate) fn get_chars(&self) -> &Vec<u32> {
        &self.chars
    }

    pub(crate) fn get_scale(&self) -> f32 {
        self.settings.scale
    }

    pub(crate) fn get_wrap(&self) -> bool {
        self.settings.wrap
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TextSettings {
    scale: f32,
    wrap: bool,
}

impl TextSettings {
    pub(crate) fn new(scale: f32, wrap: bool) -> Self {
        Self { scale, wrap }
    }

    pub(crate) fn set_wrap(&self, wrap: bool) -> Self {
        let mut clone = self.clone();
        clone.wrap = wrap;
        clone
    }
}

impl Default for TextSettings {
    fn default() -> Self {
        Self {
            scale: 0.0001,
            wrap: true,
        }
    }
}
