use super::toggle_toolbox;
use crate::{
    UIConstraintRefresh, UIElement, UIElementWrapper, APP_BUTTON_GAP_V,
    APP_BUTTON_SPACE_H, APP_BUTTON_TOP, APP_BUTTON_WIDTH_HALF, DISPLAY_EDGE_RIGHT,
};
use libremarkable::{appctx, image};

/// Creates the kebab element.
pub fn create(_app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    let x = (DISPLAY_EDGE_RIGHT - APP_BUTTON_SPACE_H) - APP_BUTTON_WIDTH_HALF;
    let y = APP_BUTTON_TOP + APP_BUTTON_GAP_V;
    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: Some(toggle_toolbox),
        inner: UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/kebab.png")).unwrap(),
        },
        ..Default::default()
    }
}
