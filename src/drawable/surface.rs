use super::Drawable;
use crate::canvas::*;
use libremarkable::input::InputEvent;

pub struct AppSurface {
    partial_refresh_rect: mxcfb_rect,
}

const APP_TOOLBAR_OFFSET: u32 = 58 + 1; // +1 so we don't clear any of the toolbar!
const APP_SURFACE_HEIGHT: u32 = (DISPLAYWIDTH - APP_TOOLBAR_OFFSET as u16) as u32;
const APP_SURFACE_WIDTH: u32 = DISPLAYWIDTH as u32;

impl AppSurface {
    pub fn new() -> Self {
        Self {
            partial_refresh_rect: mxcfb_rect {
                top: APP_TOOLBAR_OFFSET,
                left: 0,
                width: APP_SURFACE_WIDTH,
                height: APP_SURFACE_HEIGHT,
            },
        }
    }
}

impl Drawable for AppSurface {
    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {}

    fn on_input(&mut self, event: InputEvent) {}
}
