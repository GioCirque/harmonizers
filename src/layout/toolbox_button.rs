use cgmath::Point2;
use libremarkable::{
    appctx::ApplicationContext,
    ui_extensions::element::{UIConstraintRefresh, UIElement, UIElementHandle, UIElementWrapper},
};

use crate::{
    APP_BUTTON_GAP_V, APP_BUTTON_SPACE_H, APP_BUTTON_SPACE_V, APP_BUTTON_TOP, DISPLAY_EDGE_RIGHT,
};

/// Creates a toolbox button in the toolbox's rows and columns system.
pub fn create_toolbox_button(
    inner: UIElement,
    onclick: Option<fn(&mut ApplicationContext, UIElementHandle)>,
    col: u16,
    row: u16,
) -> UIElementWrapper {
    let x = DISPLAY_EDGE_RIGHT - (APP_BUTTON_SPACE_H * col) - 65;
    let y = APP_BUTTON_TOP + (APP_BUTTON_SPACE_V * row) + APP_BUTTON_GAP_V;
    UIElementWrapper {
        position: Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::RefreshAndWait,
        onclick,
        inner,
        ..Default::default()
    }
}
