use super::{is_toolbox_open, toggle_toolbox};
use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_SPACE_H, APP_BUTTON_SPACE_V,
    DISPLAY_EDGE_RIGHT, DISPLAY_EDGE_TOP,
};
use libremarkable::{
    appctx::ApplicationContext,
    image::{load_from_memory, DynamicImage},
};

/// Creates the kebab element.
pub fn create(_app: &mut ApplicationContext) -> UIElementWrapper {
    let x = DISPLAY_EDGE_RIGHT - APP_BUTTON_SPACE_H;
    let y = DISPLAY_EDGE_TOP + (APP_BUTTON_SPACE_V / 2) - 8;
    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: Some(toggle_toolbox),
        inner: UIElement::Image {
            img: get_kebab_image(is_toolbox_open()),
        },
        ..Default::default()
    }
}

pub fn get_kebab_image(open: bool) -> DynamicImage {
    match open {
        true => load_from_memory(include_bytes!("../../assets/dist/kebab-on.png")).unwrap(),
        false => load_from_memory(include_bytes!("../../assets/dist/kebab-off.png")).unwrap(),
    }
}
