use crate::{UIConstraintRefresh, UIElement, UIElementWrapper};
use libremarkable::{appctx, framebuffer::common::*};

pub fn create(_app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    UIElementWrapper {
        position: cgmath::Point2 { x: 20, y: 50 },
        refresh: UIConstraintRefresh::Refresh,
        onclick: None,
        inner: UIElement::Text {
            border_px: 0,
            foreground: color::BLACK,
            scale: 50.0,
            text: "HarmonizeRs".to_owned(),
        },
        ..Default::default()
    }
}
