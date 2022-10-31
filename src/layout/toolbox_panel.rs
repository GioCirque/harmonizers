use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_SPACE_V, APP_BUTTON_WIDTH_HALF,
    DISPLAY_EDGE_RIGHT, DISPLAY_EDGE_TOP,
};
use cgmath::Vector2;
use libremarkable::framebuffer::common::color;

const TOOLBOX_PANEL_WIDTH: u16 = 300;
const TOOLBOX_PANEL_HEIGHT: u16 = 250;

/// Creates the bordered region for the toolbox.
pub fn create() -> UIElementWrapper {
    UIElementWrapper {
        position: get_toolbox_panel_point(),
        refresh: UIConstraintRefresh::Refresh,
        onclick: None,
        inner: UIElement::Region {
            size: get_toolbox_panel_size(),
            border_color: color::BLACK,
            border_px: 2,
        },
        ..Default::default()
    }
}

pub fn get_toolbox_panel_size() -> Vector2<u32> {
    Vector2 {
        x: TOOLBOX_PANEL_HEIGHT.into(),
        y: TOOLBOX_PANEL_WIDTH.into(),
    }
}

pub fn get_toolbox_panel_point() -> cgmath::Point2<i32> {
    cgmath::Point2 {
        x: ((DISPLAY_EDGE_RIGHT - TOOLBOX_PANEL_WIDTH) + APP_BUTTON_WIDTH_HALF).into(),
        y: (DISPLAY_EDGE_TOP + (APP_BUTTON_SPACE_V / 2) + APP_BUTTON_WIDTH_HALF - 8).into(),
    }
}
