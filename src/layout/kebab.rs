use super::{is_toolbox_open, toggle_toolbox};
use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_SPACE_H, APP_BUTTON_SPACE_V,
    DISPLAY_EDGE_RIGHT, DISPLAY_EDGE_TOP,
};
use libremarkable::{
    appctx::ApplicationContext,
    framebuffer::common::mxcfb_rect,
    image::{load_from_memory, DynamicImage},
};

const KEBAB_X: u16 = DISPLAY_EDGE_RIGHT - APP_BUTTON_SPACE_H;
const KEBAB_Y: u16 = DISPLAY_EDGE_TOP + (APP_BUTTON_SPACE_V / 2) - 8;

/// Creates the kebab element.
pub fn create(_app: &mut ApplicationContext) -> UIElementWrapper {
    UIElementWrapper {
        position: cgmath::Point2 {
            x: KEBAB_X.into(),
            y: KEBAB_Y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: Some(toggle_toolbox),
        inner: UIElement::Image {
            img: get_kebab_image(is_toolbox_open()),
        },
        ..Default::default()
    }
}

pub fn get_kebab_region() -> mxcfb_rect {
    mxcfb_rect {
        top: KEBAB_Y.into(),
        left: KEBAB_X.into(),
        height: 32,
        width: 32,
    }
}

pub fn get_kebab_image(open: bool) -> DynamicImage {
    match open {
        true => load_from_memory(include_bytes!("../../assets/dist/kebab-on.png")).unwrap(),
        false => load_from_memory(include_bytes!("../../assets/dist/kebab-off.png")).unwrap(),
    }
}
