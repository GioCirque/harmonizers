use super::{AppSurface, AppToolbar, Drawable};
use crate::canvas::*;
use libremarkable::input::InputEvent;

pub struct AppChrome {
    toolbar: AppToolbar,
    surface: AppSurface,
}

impl AppChrome {
    pub fn new() -> Self {
        Self {
            toolbar: AppToolbar::new(),
            surface: AppSurface::new(),
        }
    }
}

impl Drawable for AppChrome {
    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        self.toolbar.update(canvas);
        self.surface.update(canvas);
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        self.toolbar.draw(canvas);
        self.surface.draw(canvas);
    }

    fn on_input(&mut self, event: InputEvent) {
        if self.toolbar.is_hit(&event) {
            self.toolbar.on_input(event);
        } else if self.surface.is_hit(&event) {
            self.surface.on_input(event);
        }
    }
}
