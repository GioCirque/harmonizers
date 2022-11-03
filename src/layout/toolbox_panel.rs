use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, DISPLAY_EDGE_RIGHT, DISPLAY_EDGE_TOP,
};
use cgmath::Vector2;
use libremarkable::framebuffer::common::{color, mxcfb_rect};

pub const TOOLBOX_PANEL_BORDER: u16 = 2;
pub const TOOLBOX_PANEL_WIDTH: u16 = 300;
pub const TOOLBOX_PANEL_HEIGHT: u16 = 450;

/// Creates the bordered region for the toolbox.
pub fn create() -> UIElementWrapper {
    UIElementWrapper {
        position: get_toolbox_panel_point(),
        refresh: UIConstraintRefresh::Refresh,
        onclick: None,
        inner: UIElement::Region {
            size: get_toolbox_panel_size(),
            border_color: color::BLACK,
            border_px: TOOLBOX_PANEL_BORDER.into(),
        },
        ..Default::default()
    }
}

pub fn get_toolbox_panel_region() -> mxcfb_rect {
    let size = get_toolbox_panel_size();
    let point = get_toolbox_panel_point();
    mxcfb_rect {
        top: point.y as u32,
        left: point.x as u32,
        width: size.x.into(),
        height: size.y.into(),
    }
}

pub fn get_toolbox_panel_size() -> Vector2<u32> {
    Vector2 {
        x: TOOLBOX_PANEL_WIDTH.into(),
        y: TOOLBOX_PANEL_HEIGHT.into(),
    }
}

pub fn get_toolbox_panel_point() -> cgmath::Point2<i32> {
    cgmath::Point2 {
        x: (DISPLAY_EDGE_RIGHT - TOOLBOX_PANEL_WIDTH).into(),
        y: DISPLAY_EDGE_TOP.into(),
    }
}
