use super::{toolbox_panel::TOOLBOX_PANEL_WIDTH, Point2, Vector2};
use crate::{
    appctx::ApplicationContext, color, image, UIConstraintRefresh, UIElement, UIElementHandle,
    UIElementWrapper, APP_BUTTON_GAP_H, APP_BUTTON_GAP_V, APP_BUTTON_SPACE_H, APP_DROPDOWN_TOP,
    DISPLAY_EDGE_RIGHT,
};

pub const DROPDOWN_ELEMENT_COUNT: u16 = 2;

/// Creates a toolbox button in the toolbox's rows and columns system.
pub fn create_toolbox_dropdown(
    _inner: UIElement,
    _onclick: Option<fn(&mut ApplicationContext, UIElementHandle)>,
    row: u16,
) -> Vec<UIElementWrapper> {
    let box_height: u16 = 32;
    let box_width: u16 = (f64::from(TOOLBOX_PANEL_WIDTH) * 0.8).round() as i64 as u16;
    let box_x: u16 = DISPLAY_EDGE_RIGHT - 273;
    let box_y: u16 = APP_DROPDOWN_TOP + (APP_BUTTON_SPACE_H * row) + APP_BUTTON_GAP_V;
    let arrow_x: u16 = (box_x + box_width) - (APP_BUTTON_GAP_H * 3);
    let arrow_y: u16 = APP_DROPDOWN_TOP + (APP_BUTTON_GAP_V * 2) + (APP_BUTTON_SPACE_H * row);
    let img = image::load_from_memory(include_bytes!("../../assets/dist/down.png")).unwrap();
    vec![
        UIElementWrapper {
            position: Point2 {
                x: box_x.into(),
                y: box_y.into(),
            },
            refresh: UIConstraintRefresh::RefreshAndWait,
            onclick: None,
            inner: UIElement::Region {
                size: Vector2 {
                    x: box_width as u32,
                    y: box_height as u32,
                },
                border_color: color::BLACK,
                border_px: 2,
            },
            ..Default::default()
        },
        UIElementWrapper {
            position: Point2 {
                x: arrow_x.into(),
                y: arrow_y.into(),
            },
            refresh: UIConstraintRefresh::RefreshAndWait,
            onclick: None,
            inner: UIElement::Image { img },
            ..Default::default()
        },
    ]
}
