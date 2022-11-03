mod assets;
mod buttons;
mod chrome;
mod surface;
mod toolbar;

pub use assets::Asset;
pub use buttons::*;
pub use chrome::AppChrome;
pub use surface::AppSurface;
pub use toolbar::AppToolbar;

use crate::canvas::Canvas;
use downcast_rs::Downcast;
use libremarkable::input::InputEvent;

pub trait Drawable: Downcast {
    fn draw(&mut self, canvas: &mut Canvas);
    fn update(&mut self, _canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        None
    }
    fn on_input(&mut self, _event: InputEvent) {}
}
impl_downcast!(Drawable);
