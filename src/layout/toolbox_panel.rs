use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_GAP_V, APP_BUTTON_SPACE_H,
    APP_BUTTON_TOP, APP_BUTTON_WIDTH_HALF, DISPLAY_EDGE_RIGHT,
};
use cgmath::Vector2;
use libremarkable::{appctx, framebuffer::common::color};

/// Creates the bordered region for the toolbox.
pub fn create(_app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    let width: u32 = 250;
    let height: u32 = 300;
    let x = DISPLAY_EDGE_RIGHT - (APP_BUTTON_SPACE_H - APP_BUTTON_WIDTH_HALF) - width as u16;
    let y = APP_BUTTON_TOP + APP_BUTTON_GAP_V;

    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: None,
        inner: UIElement::Region {
            size: Vector2 {
                x: width,
                y: height,
            },
            border_color: color::BLACK,
            border_px: 2,
        },
        ..Default::default()
    }
}
