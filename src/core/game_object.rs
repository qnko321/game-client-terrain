


use crate::App;


pub(crate) trait GameObject {
    /// Instantiates the game object
    fn create() -> Self where Self: Sized;

    /// Called the first time the objects gets added to the scene before the first update is called
    fn start(&mut self, app: &mut App);

    /// Called every frame
    fn update(&mut self, app: &mut App);
}