use log::info;
use crate::App;
use crate::core::game_object::GameObject;

pub(crate) struct Model {

}

impl GameObject for Model {
    fn create() -> Self {
        Self {}
    }

    fn start(&mut self, app: &mut App) {
        info!("started");
    }

    fn update(&mut self, app: &mut App) {
        info!("update");
    }
}

impl Model {
    fn test() {
        info!("test");
    }
}