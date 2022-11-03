use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_GAP_H,
    APP_BUTTON_SPACE_V, DISPLAY_EDGE_RIGHT, DISPLAY_EDGE_TOP,
};
use libremarkable::{appctx, framebuffer::common::*};

use super::toolbox_panel::TOOLBOX_PANEL_WIDTH;

pub fn create(_app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    let x = (DISPLAY_EDGE_RIGHT - TOOLBOX_PANEL_WIDTH) + APP_BUTTON_GAP_H;
    let y = (DISPLAY_EDGE_TOP + APP_BUTTON_SPACE_V) - 8;
    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: None,
        inner: UIElement::Text {
            border_px: 0,
            foreground: color::BLACK,
            scale: 35.0,
            text: "HarmonizeRs".to_owned(),
        },
        ..Default::default()
    }
}
